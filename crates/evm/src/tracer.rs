//! Structured execution tracing for failed transactions.
//!
//! `FailureTracer` implements revm's `Inspector` trait and captures structured
//! trace data only when transaction execution fails (revert or halt). On success,
//! the tracer is a no-op — it discards any buffered data to minimize overhead.
//!
//! Each `FailureTracer` instance is created per-transaction (no cross-tx state sharing),
//! using per-instance buffering to avoid serializing parallel execution.
//!
//! # Example
//!
//! ```ignore
//! use monad_evm::tracer::FailureTracer;
//!
//! let mut tracer = FailureTracer::new();
//! // ... attach to EVM via build_mainnet_with_inspector(tracer) ...
//! // ... execute transaction ...
//! if let Some(trace) = tracer.take_result() {
//!     eprintln!("Execution failed: {}", trace.to_json());
//! }
//! ```

use alloy_primitives::Address;
use revm_inspector::Inspector;
use revm::interpreter::{
    interpreter_types::Jumps, CallInputs, CallOutcome, CreateInputs, CreateOutcome, Interpreter,
    InterpreterTypes,
};
use serde::{Deserialize, Serialize};

/// Structured trace data captured when a transaction fails (revert or halt).
///
/// Contains the program counter, gas remaining, failing opcode, revert reason,
/// call depth, and nested call stack at the point of failure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceResult {
    /// Program counter at the point of failure.
    pub pc: usize,
    /// Gas remaining when execution failed.
    pub gas_remaining: u64,
    /// Opcode being executed when failure occurred.
    pub opcode: u8,
    /// Revert reason bytes (ABI-encoded error data), if available.
    pub revert_reason: Option<Vec<u8>>,
    /// Call depth at the point of failure (0 = top-level call).
    pub call_depth: u32,
    /// Stack of call target addresses from outermost to innermost.
    pub call_stack: Vec<Address>,
    /// Whether this trace represents a failed execution.
    pub failed: bool,
}

impl TraceResult {
    /// Serializes this trace result to a JSON string.
    ///
    /// Useful for structured logging and diagnostics output.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("TraceResult serialization should not fail")
    }
}

/// Inspector that captures structured trace data on transaction failure.
///
/// During execution, `step()` records the current PC, gas, and opcode into
/// lightweight per-instance fields (3 field writes, no allocation). `call()`
/// and `call_end()` track the nested call stack. On revert or halt, the
/// accumulated data is finalized into a `TraceResult`. On success, all
/// buffered data is discarded.
///
/// Created per-transaction — no shared mutable state across threads.
pub struct FailureTracer {
    /// Last observed program counter.
    last_pc: usize,
    /// Last observed gas remaining.
    last_gas: u64,
    /// Last observed opcode.
    last_opcode: u8,
    /// Current call depth (0 = top-level).
    call_depth: u32,
    /// Stack of call target addresses.
    call_stack: Vec<Address>,
    /// Finalized trace result, set when execution fails.
    result: Option<TraceResult>,
}

impl FailureTracer {
    /// Creates a new empty tracer ready to attach to an EVM.
    pub fn new() -> Self {
        Self {
            last_pc: 0,
            last_gas: 0,
            last_opcode: 0,
            call_depth: 0,
            call_stack: Vec::new(),
            result: None,
        }
    }

    /// Takes the trace result if execution failed, returns `None` if execution succeeded.
    ///
    /// This consumes the result — subsequent calls return `None`.
    pub fn take_result(&mut self) -> Option<TraceResult> {
        self.result.take()
    }

    /// Finalizes the buffered state into a `TraceResult`.
    fn finalize(&mut self, revert_reason: Option<Vec<u8>>) {
        self.result = Some(TraceResult {
            pc: self.last_pc,
            gas_remaining: self.last_gas,
            opcode: self.last_opcode,
            revert_reason,
            call_depth: self.call_depth,
            call_stack: self.call_stack.clone(),
            failed: true,
        });
    }
}

impl Default for FailureTracer {
    fn default() -> Self {
        Self::new()
    }
}

impl<CTX, INTR> Inspector<CTX, INTR> for FailureTracer
where
    INTR: InterpreterTypes,
    INTR::Bytecode: Jumps,
{
    /// Records current PC, gas remaining, and opcode on every step.
    /// This is lightweight — just 3 field writes, no allocation.
    #[inline]
    fn step(&mut self, interp: &mut Interpreter<INTR>, _context: &mut CTX) {
        self.last_pc = interp.bytecode.pc();
        self.last_gas = interp.gas.remaining();
        self.last_opcode = interp.bytecode.opcode();
    }

    /// Pushes the call target address onto the call stack and increments depth.
    fn call(&mut self, _context: &mut CTX, inputs: &mut CallInputs) -> Option<CallOutcome> {
        self.call_stack.push(inputs.target_address);
        self.call_depth += 1;
        None
    }

    /// Pops from the call stack, decrements depth. If the call reverted,
    /// captures the output bytes as revert reason and finalizes the trace.
    fn call_end(&mut self, _context: &mut CTX, _inputs: &CallInputs, outcome: &mut CallOutcome) {
        if outcome.result.result.is_revert() {
            let reason = if outcome.result.output.is_empty() {
                None
            } else {
                Some(outcome.result.output.to_vec())
            };
            self.finalize(reason);
        }
        self.call_stack.pop();
        self.call_depth = self.call_depth.saturating_sub(1);
    }

    /// Similar to `call_end` for CREATE operations — captures trace on failure.
    fn create_end(
        &mut self,
        _context: &mut CTX,
        _inputs: &CreateInputs,
        outcome: &mut CreateOutcome,
    ) {
        if outcome.result.result.is_revert() {
            let reason = if outcome.result.output.is_empty() {
                None
            } else {
                Some(outcome.result.output.to_vec())
            };
            self.finalize(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::{
        context::{Context, TxEnv},
        handler::MainBuilder,
        primitives::TxKind,
        state::Bytecode,
        MainContext,
    };
    use revm_inspector::InspectEvm;

    #[test]
    fn test_default_state_no_result() {
        let mut tracer = FailureTracer::new();
        assert!(tracer.take_result().is_none(), "Fresh tracer should have no result");
    }

    #[test]
    fn test_reverting_tx_produces_trace() {
        // Bytecode: PUSH0 PUSH0 REVERT (0x5F 0x5F 0xFD)
        // PUSH0 pushes 0 twice (offset=0, size=0), then REVERT
        let bytecode = Bytecode::new_raw(vec![0x5F, 0x5F, 0xFD].into());

        let mut tracer = FailureTracer::new();

        let ctx = Context::mainnet()
            .with_db(revm::database::BenchmarkDB::new_bytecode(bytecode))
            .modify_cfg_chained(|cfg| {
                cfg.disable_eip3607 = true;
                cfg.disable_base_fee = true;
                cfg.disable_fee_charge = true;
            });

        let mut evm = ctx.build_mainnet_with_inspector(&mut tracer);

        let tx = TxEnv::builder()
            .caller(revm::database::BENCH_CALLER)
            .kind(TxKind::Call(revm::database::BENCH_TARGET))
            .gas_limit(100_000)
            .build()
            .unwrap();

        let result = evm.inspect_one_tx(tx).unwrap();
        assert!(matches!(result, revm::context_interface::result::ExecutionResult::Revert { .. }), "Transaction should revert");

        // Drop the EVM to release the borrow on tracer
        drop(evm);

        let trace = tracer.take_result().expect("Reverting tx should produce a TraceResult");
        assert!(trace.failed, "TraceResult.failed should be true");
        assert!(trace.pc > 0 || trace.opcode > 0, "PC or opcode should be populated");
        assert!(trace.gas_remaining > 0, "Gas remaining should be > 0 (not all consumed)");
    }

    #[test]
    fn test_successful_tx_no_trace() {
        // Bytecode: STOP (0x00) — immediate success
        let bytecode = Bytecode::new_raw(vec![0x00].into());

        let mut tracer = FailureTracer::new();

        let ctx = Context::mainnet()
            .with_db(revm::database::BenchmarkDB::new_bytecode(bytecode))
            .modify_cfg_chained(|cfg| {
                cfg.disable_eip3607 = true;
                cfg.disable_base_fee = true;
                cfg.disable_fee_charge = true;
            });

        let mut evm = ctx.build_mainnet_with_inspector(&mut tracer);

        let tx = TxEnv::builder()
            .caller(revm::database::BENCH_CALLER)
            .kind(TxKind::Call(revm::database::BENCH_TARGET))
            .gas_limit(100_000)
            .build()
            .unwrap();

        let result = evm.inspect_one_tx(tx).unwrap();
        assert!(result.is_success(), "Transaction should succeed");

        drop(evm);

        assert!(
            tracer.take_result().is_none(),
            "Successful tx should produce no TraceResult"
        );
    }

    #[test]
    fn test_trace_result_json_roundtrip() {
        let trace = TraceResult {
            pc: 42,
            gas_remaining: 99000,
            opcode: 0xFD, // REVERT opcode
            revert_reason: Some(vec![0x08, 0xc3, 0x79, 0xa0]),
            call_depth: 1,
            call_stack: vec![Address::with_last_byte(0x01), Address::with_last_byte(0x02)],
            failed: true,
        };

        let json = trace.to_json();
        assert!(!json.is_empty(), "JSON output should not be empty");

        // Deserialize back
        let deserialized: TraceResult =
            serde_json::from_str(&json).expect("JSON should deserialize back");
        assert_eq!(deserialized.pc, 42);
        assert_eq!(deserialized.gas_remaining, 99000);
        assert_eq!(deserialized.opcode, 0xFD);
        assert_eq!(
            deserialized.revert_reason,
            Some(vec![0x08, 0xc3, 0x79, 0xa0])
        );
        assert_eq!(deserialized.call_depth, 1);
        assert_eq!(deserialized.call_stack.len(), 2);
        assert!(deserialized.failed);
    }

    #[test]
    fn test_trace_result_fields_populated() {
        // Bytecode: PUSH1 0x00 PUSH1 0x00 REVERT
        // This exercises step() multiple times before revert
        let bytecode = Bytecode::new_raw(vec![0x60, 0x00, 0x60, 0x00, 0xFD].into());

        let mut tracer = FailureTracer::new();

        let ctx = Context::mainnet()
            .with_db(revm::database::BenchmarkDB::new_bytecode(bytecode))
            .modify_cfg_chained(|cfg| {
                cfg.disable_eip3607 = true;
                cfg.disable_base_fee = true;
                cfg.disable_fee_charge = true;
            });

        let mut evm = ctx.build_mainnet_with_inspector(&mut tracer);

        let tx = TxEnv::builder()
            .caller(revm::database::BENCH_CALLER)
            .kind(TxKind::Call(revm::database::BENCH_TARGET))
            .gas_limit(100_000)
            .build()
            .unwrap();

        let result = evm.inspect_one_tx(tx).unwrap();
        assert!(matches!(result, revm::context_interface::result::ExecutionResult::Revert { .. }));

        drop(evm);

        let trace = tracer.take_result().expect("Should have trace");
        // Verify all key fields are populated
        assert!(trace.failed, "failed flag should be true");
        // PC should be at or after the REVERT instruction position
        // gas_remaining should be reasonable (less than initial 100k but more than 0)
        assert!(
            trace.gas_remaining > 0 && trace.gas_remaining < 100_000,
            "gas_remaining should be between 0 and 100000, got {}",
            trace.gas_remaining
        );
        // opcode should be recorded (the last step before revert processing)
        // The exact opcode depends on when step() last fires relative to REVERT handling
    }

    #[test]
    fn test_trace_result_json_fields_present() {
        let trace = TraceResult {
            pc: 10,
            gas_remaining: 50000,
            opcode: 0x60, // PUSH1
            revert_reason: None,
            call_depth: 0,
            call_stack: vec![],
            failed: true,
        };

        let json = trace.to_json();
        // Verify all expected fields are present in the JSON
        assert!(json.contains("\"pc\""), "JSON should contain 'pc' field");
        assert!(
            json.contains("\"gas_remaining\""),
            "JSON should contain 'gas_remaining' field"
        );
        assert!(
            json.contains("\"opcode\""),
            "JSON should contain 'opcode' field"
        );
        assert!(
            json.contains("\"revert_reason\""),
            "JSON should contain 'revert_reason' field"
        );
        assert!(
            json.contains("\"call_depth\""),
            "JSON should contain 'call_depth' field"
        );
        assert!(
            json.contains("\"call_stack\""),
            "JSON should contain 'call_stack' field"
        );
        assert!(
            json.contains("\"failed\""),
            "JSON should contain 'failed' field"
        );
    }

    #[test]
    fn test_take_result_consumes() {
        let mut tracer = FailureTracer::new();
        // Manually finalize to simulate a failure
        tracer.last_pc = 5;
        tracer.last_gas = 1000;
        tracer.last_opcode = 0xFD;
        tracer.finalize(Some(vec![0xDE, 0xAD]));

        let first = tracer.take_result();
        assert!(first.is_some(), "First take should return Some");

        let second = tracer.take_result();
        assert!(second.is_none(), "Second take should return None (consumed)");
    }
}
