---
estimated_steps: 3
estimated_files: 5
---

# T02: Unify all Korean UX copy to English across 5 component files

**Slice:** S03 — Motion + Polish — 애니메이션, 모바일, UX 카피
**Milestone:** M004

## Description

Replace all ~35 Korean user-facing strings with English equivalents across 5 component files, and change the `<html lang="ko">` to `<html lang="en">`. Vibe-Loom targets the international Monad developer ecosystem, so all rendered text must be English. JSDoc comments and inline code comments in Korean are intentionally preserved — only strings that render in the browser need translation.

## Steps

1. **Replace Korean strings in `src/app/page.tsx`** (13 strings):
   - Line 29: `desc: "배포 에러 테스트"` → `desc: "Deploy error test"`
   - Line 30: `desc: "수정된 버전"` → `desc: "Fixed version"`
   - Line 31: `desc: "Pectra 옵코드 테스트"` → `desc: "Pectra opcode test"`
   - Line 32: `desc: "병렬 실행 테스트"` → `desc: "Parallel execution test"`
   - Line 126: `"배포할 컨트랙트 소스가 없습니다."` → `"No contract source to deploy."`
   - Line 160: `"배포 중 오류가 발생했습니다."` → `"An error occurred during deployment."`
   - Line 242: `"컴파일 중..."` → `"Compiling..."`
   - Line 249: `"배포 중..."` → `"Deploying..."`
   - Line 256: `"분석 중..."` → `"Analyzing..."`
   - Line 268: `로그아웃` → `Logout`
   - Line 292: `배포 완료` → `Deploy Complete`
   - Line 317: `AI가 수정 중...` → `AI fixing...`
   - Line 323: `AI 수정 제안` → `AI Fix Suggestion`
   - **DO NOT touch** line 174 (`// 무시`) — that's a code comment, not rendered text

2. **Replace Korean strings in remaining component files** (22 strings total):
   - **`src/components/VibeStatus.tsx`** (4 rendered strings — leave JSDoc on lines 8-10 as-is):
     - Line 40: `로그인 필요` → `Login required`
     - Line 49: `로딩 중...` → `Loading...`
     - Line 68: `무료 배포` → `free deploys`
     - Line 73: `지갑 연결 필요` → `Wallet required`
   - **`src/components/WalletConnectModal.tsx`** (10 strings):
     - Line 114: `🔗 지갑 연결 배포` → `🔗 Wallet Deploy`
     - Line 126: `무료 배포 횟수를 초과했습니다. 지갑을 연결하여 직접 배포하세요.` → `Free deploy quota exceeded. Connect your wallet to deploy directly.`
     - Line 133: `지갑을 연결하세요:` → `Connect a wallet:`
     - Line 152: `연결된 지갑` → `Connected wallet`
     - Line 161: `연결 해제` → `Disconnect`
     - Line 172: `컴파일 중...` → `Compiling...`
     - Line 174: `트랜잭션 전송 중...` → `Sending transaction...`
     - Line 176: `트랜잭션 확인 중...` → `Confirming transaction...`
     - Line 183: `배포 완료!` → `Deploy complete!`
   - **`src/components/ide/VibeScoreDashboard.tsx`** (5 strings):
     - Line 50: `Monad 병렬 실행에 적합` → `Optimized for Monad parallel execution`
     - Line 52: `일부 최적화 권장` → `Some optimization recommended`
     - Line 53: `성능 저하 위험 요소 존재` → `Performance risk factors detected`
     - Line 94: `분석 중...` → `Analyzing...`
     - Line 187: `개선 제안` → `Suggestions`

3. **Change html lang attribute in `src/app/layout.tsx`**:
   - Change `<html lang="ko"` to `<html lang="en"`

## Must-Haves

- [ ] All 35 Korean rendered strings replaced with English
- [ ] `<html lang="en">` in layout.tsx
- [ ] JSDoc comments in Korean preserved (VibeStatus.tsx lines 8-10)
- [ ] Inline comments in Korean preserved (page.tsx `// 무시`)
- [ ] Build passes with zero errors

## Verification

- `npm run build` exits 0 (run from `/home/ahwlsqja/Vibe-Loom`)
- `grep -q 'lang="en"' src/app/layout.tsx` exits 0
- `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/app/page.tsx | grep -v "//"` returns empty (all Korean is in comments)
- `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/components/VibeStatus.tsx | grep -v "^\s*\*\|^\s*//"` returns empty (Korean only in JSDoc)
- `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/components/WalletConnectModal.tsx` returns empty
- `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/components/ide/VibeScoreDashboard.tsx` returns empty
- `grep -q 'Compiling\.\.\.' src/app/page.tsx` exits 0
- `grep -q 'Deploy complete!' src/components/WalletConnectModal.tsx` exits 0

## Inputs

- `src/app/page.tsx` — 394 lines; contains 13 Korean rendered strings (after T01 has applied btn-press classes)
- `src/components/VibeStatus.tsx` — 87 lines; 4 Korean rendered strings + 3 JSDoc comment lines
- `src/components/WalletConnectModal.tsx` — 201 lines; 10 Korean rendered strings
- `src/components/ide/VibeScoreDashboard.tsx` — 205 lines; 5 Korean rendered strings
- `src/app/layout.tsx` — 37 lines; `lang="ko"` on html element

## Expected Output

- `src/app/page.tsx` — all 13 Korean strings replaced with English equivalents
- `src/components/VibeStatus.tsx` — 4 rendered Korean strings replaced; JSDoc preserved
- `src/components/WalletConnectModal.tsx` — 10 Korean strings replaced
- `src/components/ide/VibeScoreDashboard.tsx` — 5 Korean strings replaced
- `src/app/layout.tsx` — `lang="en"` on html element
