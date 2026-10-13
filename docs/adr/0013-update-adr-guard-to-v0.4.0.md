# ADR 0013: adr-guard を v0.4.0 に上げ、ADRの参照のリンクを検査する

ステータス: 採択→指定するバージョンとステータスの行の許可は[ADR 0014](0014-update-adr-guard-to-v0.5.0.md)

## 文脈

[ADR 0012](0012-update-adr-guard-to-v0.3.0.md) で、CIの `adr` ジョブの adr-guard のactionを v0.3.0 とした。

その後、tett23/adr-guard は v0.4.0 に更新された（tett23/adr-guard の [ADR 0009](https://github.com/tett23/adr-guard/blob/main/docs/adr/0009-require-links-for-adr-references.md)）。
v0.4.0 では、文書（`.md`）の中のADRの参照（`ADR 0001` の形）にリンクが無いと、pre-commit hookとCIでエラーになる。
あわせて、リンクを自動で付ける `adr-guard links --fix` と、コミット済みのADRにリンクを付けるだけの変更を許可する `--allow-links` が加わった。

このリポジトリでは、ADRの参照をすでにリンクにしている（`655de1b` のマージ）。
v0.4.0 の `adr-guard links` で検査すると、リンクの無い参照は [ADR 0011](0011-pin-adr-guard-action-by-version.md) のステータスの行にある `ADR 0012` の1件だけだった。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.4.0` にし、入力 `allow-links: true` を付ける。入力 `links` は既定（`true`）のままとし、リンクの検査を実行する。
- `.githooks/pre-commit` を `exec adr-guard pre-commit --allow-links` にする。
- [ADR 0011](0011-pin-adr-guard-action-by-version.md) のステータスの行の `ADR 0012` をリンクにする。ステータスの行の変更のため、`--allow-links` は要らない。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.4.0 にし、ADRの参照にはリンクが要ること、`adr-guard links --fix` で修正できることを記載する。意図してADRを書きかえる場合の手順は、リンクを付けるだけなら `--allow-links` で足り、トレーラーは緊急用であることがわかるように直す。
- `docs/specifications.md` のADRとCIの記述を v0.4.0 に合わせる（リンクの検査、`--allow-links`）。
- [ADR 0012](0012-update-adr-guard-to-v0.3.0.md) のステータスを `採択→指定するバージョンは[ADR 0013](0013-update-adr-guard-to-v0.4.0.md)` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- 手元の adr-guard の更新の自動化。各自がREADMEの手順で更新する。
- Claude Codeのhook（全体の設定 `~/.claude/settings.json`）の変更。`claude-hook` は v0.4.0 でも挙動が変わらない。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.4.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.4.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-links` と `links` が実行されていることを確かめる。
- 手元の adr-guard を v0.4.0 にしたうえで、次を確かめる。
  - `adr-guard links` が終了コード0で終わる。
  - リンクの無い参照を含む文書をステージすると、pre-commit hookで拒否される（コミットはしない）。

## トレードオフ

- pre-commit hookに `--allow-links` を付けるため、手元の adr-guard が v0.4.0 より古いと、引数の誤り（終了コード64）でコミットが止まる。READMEのインストール手順のタグを上げて補う。
- リンクの検査により、ADRの参照にリンクを付け忘れるとコミットとCIが止まる。`adr-guard links --fix` で直せるため、手間は小さい。
- ほかのリポジトリのADRへの参照（tett23/adr-guard の [ADR 0009](https://github.com/tett23/adr-guard/blob/main/docs/adr/0009-require-links-for-adr-references.md) など）は、このリポジトリに同じ番号のADRがあると、このリポジトリのADRへの参照として扱われる。ほかのリポジトリのADRへの参照には、手でリンクを付ける。
