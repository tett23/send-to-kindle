# 仕様

## 開発プロセス

### ADR

- 実装の前に `docs/adr/` にADRを作成し、人間のレビューを経てからコミットする。
- ファイル名は `0001-feature-details.md` の形式（4桁の連番＋kebab-case）とする。
- コミット済みのADRはステータスの行を除いて変更できない。仕様を変更する場合は新しいADRを作成する。
- この制約は次の二つのhookで強制する。検査はRust製の `tools/adr-guard` が担う（ADR 0001）。
  - gitのpre-commit hook（`.githooks/pre-commit`）
  - Claude CodeのPreToolUse hook（`.claude/settings.json`）

## 破棄したADR

なし
