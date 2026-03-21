//! Test vectors for all 9 standard Ethereum precompiles.
//!
//! Each test calls the precompile function directly (via revm's precompile API)
//! with known input/output pairs, verifying byte-level correctness.
//!
//! These tests validate the precompile implementations independently from the
//! full EVM execution pipeline. Integration tests in monad-evm verify the
//! pipeline end-to-end.

use revm::precompile::{
    blake2, bn254, hash, identity, modexp, secp256k1,
};

// ─── 0x01: ecrecover ───────────────────────────────────────────────────────

/// Test vector from Ethereum's official ecrecover test:
/// Given a known message hash, v, r, s → recover the correct signer address.
///
/// This uses the well-known Ethereum test vector:
/// - Message hash: keccak256 of a known message
/// - Signature components: v=28, r, s from a deterministic signing
///
/// Input format: [32 bytes hash][32 bytes v][32 bytes r][32 bytes s] = 128 bytes
#[test]
fn ecrecover_valid_signature() {
    // Test vector from go-ethereum / EIP tests
    // msg hash: 0x456e9aea5e197a1f1af7a3e85a3212fa4049a3ba34c2289b4c860fc0b0c64ef3
    // v: 28
    // r: 0x9242685bf161793cc25603c231bc2f568eb630ea16aa137d2664ac8038825608
    // s: 0x4f8ae3bd7535248d0bd448298cc2e2071e56992d0774dc340c368ae950852ada
    // Expected recovered address: 0x7156526fbd7a3c72969b54f64e42c10fbb768c8a

    let mut input = [0u8; 128];

    // msg hash (32 bytes)
    let hash = hex::decode("456e9aea5e197a1f1af7a3e85a3212fa4049a3ba34c2289b4c860fc0b0c64ef3").unwrap();
    input[..32].copy_from_slice(&hash);

    // v = 28 (32 bytes, big-endian, value 28 in last byte)
    input[63] = 28;

    // r (32 bytes)
    let r = hex::decode("9242685bf161793cc25603c231bc2f568eb630ea16aa137d2664ac8038825608").unwrap();
    input[64..96].copy_from_slice(&r);

    // s (32 bytes)
    let s = hex::decode("4f8ae3bd7535248d0bd448298cc2e2071e56992d0774dc340c368ae950852ada").unwrap();
    input[96..128].copy_from_slice(&s);

    let result = secp256k1::ec_recover_run(&input, 10_000).unwrap();
    assert!(!result.bytes.is_empty(), "ecrecover should return 32 bytes");
    assert_eq!(result.bytes.len(), 32);

    // The recovered address is left-padded to 32 bytes (first 12 bytes are 0)
    let recovered = &result.bytes[12..32];
    let expected = hex::decode("7156526fbd7a3c72969b54f64e42c10fbb768c8a").unwrap();
    assert_eq!(recovered, expected.as_slice(), "recovered address mismatch");
    assert_eq!(result.gas_used, 3000);
}

#[test]
fn ecrecover_invalid_v_returns_empty() {
    // Invalid v value (not 27 or 28) should return empty output
    let mut input = [0u8; 128];
    input[63] = 26; // invalid v

    let result = secp256k1::ec_recover_run(&input, 10_000).unwrap();
    assert!(result.bytes.is_empty(), "invalid v should return empty bytes");
    assert_eq!(result.gas_used, 3000);
}

// ─── 0x02: SHA-256 ─────────────────────────────────────────────────────────

#[test]
fn sha256_hello() {
    // SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
    let input = b"hello";
    let result = hash::sha256_run(input, 10_000).unwrap();

    let expected = hex::decode("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824").unwrap();
    assert_eq!(result.bytes.as_ref(), expected.as_slice());
    // Gas: base=60, word=12; "hello" is 5 bytes = 1 word → 60 + 12 = 72
    assert_eq!(result.gas_used, 72);
}

#[test]
fn sha256_empty_input() {
    // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let input = b"";
    let result = hash::sha256_run(input, 10_000).unwrap();

    let expected = hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap();
    assert_eq!(result.bytes.as_ref(), expected.as_slice());
    assert_eq!(result.gas_used, 60); // base only, 0 words
}

// ─── 0x03: RIPEMD-160 ──────────────────────────────────────────────────────

#[test]
fn ripemd160_hello() {
    // RIPEMD-160("hello") = 108f07b8382412612c048d07d13f814118445acd
    // Output is left-padded to 32 bytes
    let input = b"hello";
    let result = hash::ripemd160_run(input, 10_000).unwrap();

    // revm returns the hash left-padded to 32 bytes
    let mut expected = [0u8; 32];
    let hash_bytes = hex::decode("108f07b8382412612c048d07d13f814118445acd").unwrap();
    expected[12..32].copy_from_slice(&hash_bytes);
    assert_eq!(result.bytes.as_ref(), &expected);
    // Gas: base=600, word=120; "hello" is 5 bytes = 1 word → 600 + 120 = 720
    assert_eq!(result.gas_used, 720);
}

// ─── 0x04: identity ────────────────────────────────────────────────────────

#[test]
fn identity_passthrough() {
    let input = b"arbitrary data here 0123456789";
    let result = identity::identity_run(input, 10_000).unwrap();
    assert_eq!(result.bytes.as_ref(), input);
}

#[test]
fn identity_empty() {
    let result = identity::identity_run(b"", 10_000).unwrap();
    assert!(result.bytes.is_empty());
    assert_eq!(result.gas_used, 15); // base=15, 0 words
}

// ─── 0x05: modexp ───────────────────────────────────────────────────────────

/// modexp: base=2, exp=10, mod=1000 → 2^10 mod 1000 = 1024 mod 1000 = 24
///
/// Input format: [base_len (32 bytes)][exp_len (32 bytes)][mod_len (32 bytes)][base][exp][mod]
#[test]
fn modexp_basic_exponentiation() {
    // base_len = 1, exp_len = 1, mod_len = 2
    let mut input = Vec::new();

    // base_len = 1 (32 bytes, big-endian)
    input.extend_from_slice(&[0u8; 31]);
    input.push(1);

    // exp_len = 1 (32 bytes, big-endian)
    input.extend_from_slice(&[0u8; 31]);
    input.push(1);

    // mod_len = 2 (32 bytes, big-endian)
    input.extend_from_slice(&[0u8; 31]);
    input.push(2);

    // base = 2 (1 byte)
    input.push(2);

    // exp = 10 (1 byte)
    input.push(10);

    // mod = 1000 (2 bytes, big-endian) = 0x03E8
    input.push(0x03);
    input.push(0xE8);

    // Use Berlin pricing (active in CANCUN)
    let result = modexp::berlin_run(&input, 100_000).unwrap();
    // 2^10 mod 1000 = 1024 mod 1000 = 24 → 0x0018 (2 bytes, left-padded to mod_len=2)
    assert_eq!(result.bytes.as_ref(), &[0x00, 0x18]);
}

/// modexp with modulus = 1 should return 0 (any number mod 1 = 0)
#[test]
fn modexp_modulus_one() {
    let mut input = Vec::new();

    // base_len = 1
    input.extend_from_slice(&[0u8; 31]);
    input.push(1);
    // exp_len = 1
    input.extend_from_slice(&[0u8; 31]);
    input.push(1);
    // mod_len = 1
    input.extend_from_slice(&[0u8; 31]);
    input.push(1);

    // base = 5
    input.push(5);
    // exp = 3
    input.push(3);
    // mod = 1
    input.push(1);

    let result = modexp::berlin_run(&input, 100_000).unwrap();
    // 5^3 mod 1 = 0
    assert_eq!(result.bytes.as_ref(), &[0x00]);
}

/// modexp with empty/zero lengths should return empty bytes
#[test]
fn modexp_zero_lengths() {
    let mut input = Vec::new();

    // base_len = 0
    input.extend_from_slice(&[0u8; 32]);
    // exp_len = 0
    input.extend_from_slice(&[0u8; 32]);
    // mod_len = 0
    input.extend_from_slice(&[0u8; 32]);

    let result = modexp::berlin_run(&input, 100_000).unwrap();
    assert!(result.bytes.is_empty());
}

// ─── 0x06: ecAdd (BN128 curve point addition) ──────────────────────────────

/// ecAdd: P + 0 = P (adding the point at infinity returns the same point)
///
/// BN128 generator point G = (1, 2)
/// Input: [x1 (32 bytes)][y1 (32 bytes)][x2 (32 bytes)][y2 (32 bytes)]
/// Adding G + (0,0) should return G
#[test]
fn ec_add_point_plus_identity() {
    let mut input = [0u8; 128];

    // P = generator (1, 2)
    input[31] = 1; // x1 = 1
    input[63] = 2; // y1 = 2
    // Q = (0, 0) — point at infinity
    // x2 and y2 are already 0

    let result = bn254::run_add(
        &input,
        bn254::add::ISTANBUL_ADD_GAS_COST,
        100_000,
    ).unwrap();

    // Result should be P = (1, 2)
    assert_eq!(result.bytes.len(), 64);
    let mut expected = [0u8; 64];
    expected[31] = 1; // x = 1
    expected[63] = 2; // y = 2
    assert_eq!(result.bytes.as_ref(), &expected);
}

/// ecAdd: G + G = 2G (doubling the generator)
///
/// BN128 G = (1, 2)
/// 2G = (0x030644...a0d8, 0x15ed7...3fb0)
#[test]
fn ec_add_generator_doubling() {
    let mut input = [0u8; 128];

    // P = G = (1, 2)
    input[31] = 1;
    input[63] = 2;
    // Q = G = (1, 2)
    input[95] = 1;
    input[127] = 2;

    let result = bn254::run_add(
        &input,
        bn254::add::ISTANBUL_ADD_GAS_COST,
        100_000,
    ).unwrap();

    assert_eq!(result.bytes.len(), 64);

    // 2G on alt_bn128:
    // x = 0x030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3
    // y = 0x15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4
    let expected_x = hex::decode("030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3").unwrap();
    let expected_y = hex::decode("15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4").unwrap();
    assert_eq!(&result.bytes[..32], expected_x.as_slice(), "2G x-coordinate mismatch");
    assert_eq!(&result.bytes[32..64], expected_y.as_slice(), "2G y-coordinate mismatch");
}

// ─── 0x07: ecMul (BN128 scalar multiplication) ─────────────────────────────

/// ecMul: G * 1 = G
///
/// Multiplying the generator by 1 should return the generator.
#[test]
fn ec_mul_by_one() {
    let mut input = [0u8; 96];

    // P = G = (1, 2)
    input[31] = 1;
    input[63] = 2;
    // scalar = 1
    input[95] = 1;

    let result = bn254::run_mul(
        &input,
        bn254::mul::ISTANBUL_MUL_GAS_COST,
        100_000,
    ).unwrap();

    assert_eq!(result.bytes.len(), 64);
    let mut expected = [0u8; 64];
    expected[31] = 1;
    expected[63] = 2;
    assert_eq!(result.bytes.as_ref(), &expected, "G * 1 should equal G");
}

/// ecMul: G * 2 = 2G (same as G+G from ecAdd test)
#[test]
fn ec_mul_by_two() {
    let mut input = [0u8; 96];

    // P = G = (1, 2)
    input[31] = 1;
    input[63] = 2;
    // scalar = 2
    input[95] = 2;

    let result = bn254::run_mul(
        &input,
        bn254::mul::ISTANBUL_MUL_GAS_COST,
        100_000,
    ).unwrap();

    assert_eq!(result.bytes.len(), 64);

    // Same 2G as ecAdd test
    let expected_x = hex::decode("030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3").unwrap();
    let expected_y = hex::decode("15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4").unwrap();
    assert_eq!(&result.bytes[..32], expected_x.as_slice());
    assert_eq!(&result.bytes[32..64], expected_y.as_slice());
}

// ─── 0x08: ecPairing (BN128 pairing check) ─────────────────────────────────

/// ecPairing with empty input returns 1 (trivially valid pairing)
///
/// The specification says: if the input is empty (no pairs), return 1.
#[test]
fn ec_pairing_empty_input_returns_one() {
    let input = [];

    let result = bn254::run_pair(
        &input,
        bn254::pair::ISTANBUL_PAIR_BASE,
        bn254::pair::ISTANBUL_PAIR_PER_POINT,
        100_000,
    ).unwrap();

    // Result is a 32-byte big-endian integer: 1 if pairing check passes
    let mut expected = [0u8; 32];
    expected[31] = 1;
    assert_eq!(result.bytes.as_ref(), &expected, "empty pairing should return 1");
}

/// ecPairing: e(P1, Q1) * e(P2, Q2) = 1 (valid pairing check)
///
/// Uses the well-known Ethereum test vector for bn256Pairing from
/// go-ethereum's reference tests. This tests e(G1, G2) * e(-G1, G2) = 1.
///
/// Each pair is 192 bytes: [G1_x (32)][G1_y (32)][G2_x_imag (32)][G2_x_real (32)][G2_y_imag (32)][G2_y_real (32)]
#[test]
fn ec_pairing_valid_check() {
    // Standard BN256 pairing test from Ethereum reference tests:
    // Pair 1: G1 generator (1,2) with G2 generator
    // Pair 2: -G1 generator (1, p-2) with G2 generator
    // Result: e(G1,G2) * e(-G1,G2) = e(G1,G2) * e(G1,G2)^-1 = 1
    let input = hex::decode(concat!(
        // Pair 1: G1 = (1, 2)
        "0000000000000000000000000000000000000000000000000000000000000001",
        "0000000000000000000000000000000000000000000000000000000000000002",
        // G2 generator: [x_c1][x_c0][y_c1][y_c0]
        "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2",
        "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
        "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
        "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
        // Pair 2: -G1 = (1, p-2) where p = field prime
        "0000000000000000000000000000000000000000000000000000000000000001",
        "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45",
        // Same G2 generator
        "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2",
        "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
        "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
        "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"
    )).unwrap();

    assert_eq!(input.len(), 384, "2 pairs × 192 bytes = 384");

    let result = bn254::run_pair(
        &input,
        bn254::pair::ISTANBUL_PAIR_BASE,
        bn254::pair::ISTANBUL_PAIR_PER_POINT,
        500_000,
    ).unwrap();

    // Pairing check should pass → returns 1
    let mut expected = [0u8; 32];
    expected[31] = 1;
    assert_eq!(result.bytes.as_ref(), &expected, "valid pairing should return 1");
}

// ─── 0x09: blake2f ─────────────────────────────────────────────────────────

/// blake2f test vector from EIP-152
///
/// Input format: [4 bytes rounds][64 bytes h][128 bytes m][16 bytes t][1 byte f]
/// Total: 213 bytes
#[test]
fn blake2f_eip152_vector() {
    // EIP-152 test vector #1 (from the EIP spec)
    // rounds = 12 (0x0000000c)
    // h = state vector, m = message block, t = counter, f = final block
    let input = hex::decode(concat!(
        "0000000c",
        "48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5",
        "d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b",
        "6162630000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "03000000000000000000000000000000",
        "01"
    )).unwrap();

    assert_eq!(input.len(), 213, "blake2f input must be exactly 213 bytes");

    let result = blake2::run(&input, 100_000).unwrap();

    // Expected output from EIP-152 (64 bytes = new state after compression)
    let expected = hex::decode(concat!(
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1",
        "7d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
    )).unwrap();

    assert_eq!(result.bytes.as_ref(), expected.as_slice(), "blake2f output mismatch");
    assert_eq!(result.gas_used, 12, "gas should equal number of rounds");
}

#[test]
fn blake2f_zero_rounds() {
    // 0 rounds should still work, output the same as input state
    let input = hex::decode(concat!(
        "00000000",
        "48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5",
        "d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b",
        "6162630000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "03000000000000000000000000000000",
        "01"
    )).unwrap();

    let result = blake2::run(&input, 100_000).unwrap();
    assert_eq!(result.gas_used, 0, "0 rounds = 0 gas");
    assert_eq!(result.bytes.len(), 64);
}
