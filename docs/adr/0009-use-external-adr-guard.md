# ADR 0009: 独立したリポジトリの adr-guard を使う

ステータス: 採択→actionの指定方法はADR 0011

## 文脈

ADR 0001 と ADR 0004 で、コミット済みのADRの変更を防ぐ `adr-guard` を、このリポジトリの `tools/adr-guard` に作った。
ADRを使う運用はほかのリポジトリでも同じであり、リポジトリごとに複製すると保守の手間がかかる。
そこで adr-guard を独立したリポジトリ [tett23/adr-guard](https://github.com/tett23/adr-guard) に移し、リリースとCI用のcomposite actionで配布する（tett23/adr-guard の ADR 0001）。

このリポジトリも、`tools/adr-guard` をやめて独立した adr-guard を使うように切りかえる。

## 実装すること

- tett23/adr-guard の最初のリリースが作成されたあとに、次を行う。
- `tools/adr-guard` を削除する。`.gitignore` の `/tools/adr-guard/target/` も削除する。
- gitのpre-commit hook（`.githooks/pre-commit`）を `exec adr-guard pre-commit` にする。
- Claude CodeのPreToolUse hook（`.claude/settings.json`）のコマンドを `adr-guard claude-hook` にする。
- CI（`.github/workflows/ci.yml`）
  - `rust` ジョブのマトリクスから `adr-guard` を外し、リポジトリ直下だけを検査する。
  - `adr` ジョブを、tett23/adr-guard のcomposite action（`uses: tett23/adr-guard@<タグ>`、コミットのSHAで固定する）に置きかえる。範囲（`base`、`head`）はADR 0004 のとおりとする。
- `README.md` の開発の節に、adr-guard のインストール手順（tett23/adr-guard のREADMEへのリンク）を記載し、hookの実行にRustのツールチェインが不要になったことを反映する。
- `docs/specifications.md` の開発プロセスの記述を更新する。
- ADR 0001 のステータスを `採択→0004、0009` に、ADR 0004 のステータスを `採択→0006、0009` にする。

## 実装しないこと

- adr-guard の挙動の変更。
- adr-guard のバージョンを手元とCIでそろえる仕組み。
- 手元に adr-guard が無いときに、自動でインストールする仕組み。

## テスト設計

- 切りかえのプルリクエストのCIで、`rust` ジョブとcomposite actionによる `adr` ジョブが成功することを確かめる。
- 手元で adr-guard をインストールした状態で、次を確かめる。
  - コミット済みのADRの本文を変えたコミットが、pre-commit hookで拒否される。
  - コミット済みのADRの本文を変えるClaude Codeの編集が、hookで拒否される（`adr-guard claude-hook` に標準入力でJSONを与えて、終了コード2を確かめる）。
- 切りかえのプルリクエストに含まれるコミットが、check-rangeの検査を通る（`tools/adr-guard` の削除はADRの変更ではないため）。

## トレードオフ

- 開発に参加する人は、adr-guard を別途インストールする必要がある。これまではRustのツールチェインがあればhookが動いた。リリースのバイナリを使えばRustのツールチェインは不要になる。
- adr-guard が手元に無いと、pre-commit hookはコミットを拒否し、Claude Codeのhookは編集を止めない（フェイルオープン）。どちらの場合もCIで検査されるため、変更が `main` に入ることは防げる。
- 手元の adr-guard とCIの adr-guard のバージョンが異なりうる。判定の規則はめったに変わらないため、影響は小さい。
