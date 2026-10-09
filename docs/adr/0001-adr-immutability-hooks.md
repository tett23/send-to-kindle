# ADR 0001: ADRの運用とコミット済みADRの変更防止

ステータス: 採択

## 文脈

このリポジトリでは、実装の前に必ずADRを作成し、人間のレビューを経てからコミットする。
コミット済みのADRは決定の記録であり、あとから書きかえると経緯が失われる。
そのため、ステータスの行を除いて変更できないことを、運用ルールだけでなく仕組みで保証したい。
変更の経路は、Claude Codeによる編集と、人間を含むgitへのコミットの二つがある。

## 実装すること

- ADRは `docs/adr/` に `0001-feature-details.md` の形式（4桁の連番＋kebab-case）で配置する。
- 検査はRustで実装した単一のバイナリ `adr-guard`（`tools/adr-guard/`）が担う。
  - リポジトリ本体のCargoワークスペースとは独立したクレートとする。
  - hookからは `cargo run --quiet --manifest-path tools/adr-guard/Cargo.toml -- <サブコマンド>` で起動する。
  - 比較の前に `ステータス:` で始まる行を `ステータス:` に置きかえて正規化する。正規化後に差分があれば変更とみなす。
- `adr-guard pre-commit`：gitのpre-commit hook（`.githooks/pre-commit`）から呼ぶ。
  - `core.hooksPath` を `.githooks` に設定して有効化する。
  - ステージされた `docs/adr/` 以下の変更のうち、HEADに存在するファイルについて、HEADとインデックスの内容を比較する。差分があればコミットを拒否する。
  - コミット済みADRの削除・リネームは拒否する。新規追加は許可する。
  - HEADがない初回コミットでは検査しない。
- `adr-guard claude-hook`：Claude CodeのPreToolUse hookとして `.claude/settings.json` に登録する。
  - Write・Edit・MultiEditを対象とする。
  - 対象ファイルが `docs/adr/` 以下で、かつHEADに存在する場合、ツール実行後の内容を計算してHEADと比較する。差分があれば終了コード2で拒否する。
- ADRを破棄した場合は `docs/specifications.md` に経緯と理由を記載し、その連番は欠番とする。
- ADRにより仕様が変わったら、`docs/specifications.md` と `README.md` を同時に更新する。

## 実装しないこと

- Bashツール経由（`sed -i` や `rm` など）でのClaude Codeによる変更の検知。これはpre-commit hookで捕捉する。
- `git commit --no-verify` による回避の防止。
- CIでの検査。
- 未コミットのADRの編集制限。
- ビルド済みバイナリの配布。各環境で `cargo run` によりビルドする。

## テスト設計

`tools/adr-guard` で `cargo test` を実行する。

- 単体テスト
  - ステータス行のみの差分は許可、本文や末尾の改行の差分は拒否と判定される。
  - Editは最初の一致のみ、`replace_all` では全件を置換する。MultiEditは順に適用される。
  - パスの `.` と `..` が字句的に除去される。
- 結合テスト（一時ディレクトリにgitリポジトリを作り、バイナリを実行する）
  - pre-commit
    - コミット済みADRのステータス行のみの変更はコミットできる。
    - コミット済みADRの本文の変更はコミットが拒否される。
    - コミット済みADRの削除・リネームはコミットが拒否される。
    - 新規ADRの追加はコミットできる。
    - HEADがない初回コミットは検査をスキップする。
  - claude-hook（標準入力にJSONを与える）
    - コミット済みADRへのステータス行のみのEditは終了コード0。
    - コミット済みADRの本文を変えるEdit（絶対パス指定）・Write・MultiEditは終了コード2。
    - 未コミットのADR、および `docs/adr/` 以外のファイルは終了コード0。

## トレードオフ

- `core.hooksPath` はクローンごとにローカル設定が必要で、設定を忘れるとpre-commit hookが働かない。READMEに手順を記載して補う。
- hookの実行にRustツールチェインが必要になる。初回はビルドに数秒かかるが、以降はビルド済みのバイナリが使われる。
- `adr-guard` のビルドに失敗した場合、Claude Code hookは終了コード2以外を返すため、編集は拒否されない（フェイルオープン）。pre-commit hookはビルド失敗でコミットが拒否される。
- ステータスの行は `ステータス:` で始まる行として判定するため、本文にその形式の行を追加・変更した場合も許可されてしまう。実運用上の影響は小さいと判断した。
