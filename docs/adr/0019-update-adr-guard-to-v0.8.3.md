# ADR 0019: adr-guard を v0.8.3 に上げ、ステータス `採択（未実装）` を扱えるようにする

ステータス: 採択

## 文脈

[ADR 0018](0018-update-adr-guard-to-v0.8.2.md) で、CIの `adr` ジョブの adr-guard のactionを v0.8.2 とした。

その後、tett23/adr-guard は v0.8.3 に更新された（tett23/adr-guard の [ADR 0016: 採択（未実装）のステータス](https://github.com/tett23/adr-guard/blob/main/docs/adr/0016-accepted-not-implemented-status.md)、[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.3)）。

- 新しいステータス `採択（未実装）`（英語は `Accepted (not implemented)`。括弧は全角でも半角でもよい）が加わった。採択されたが、まだ実装されていないADRを表す。
- 新しいサブコマンド `adr-guard unimplemented [--status]` が加わった。ステータスが `採択（未実装）` のADRを一覧する。

このリポジトリの文書は、ADRのステータスの候補を列挙していない。また、ADRのステータスの書式を検査するテストも無い。
手元の adr-guard はすでに v0.8.3 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.3` にする。入力 `allow-status: true` と `allow-links: true` はそのままとする。
  - actionの上のADRの番号を並べたコメントに 0019 を加える。
- `.githooks/pre-commit` は `exec adr-guard pre-commit --allow-status --allow-links` のままとし、コメントのADRの番号に 0019 を加える。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.8.3 にし、採択されたが未実装のADRのステータスを `採択（未実装）` とし、`adr-guard unimplemented` で一覧できることを書く。
- `docs/specifications.md` のCIの記述を v0.8.3 に合わせ、ステータス `採択（未実装）` と `adr-guard unimplemented` を書く。
- [ADR 0018](0018-update-adr-guard-to-v0.8.2.md) のステータスを `採択→指定するバージョンは[ADR 0019](0019-update-adr-guard-to-v0.8.3.md)` にする。
- `adr-guard links --fix --include-committed` を実行し、リンクの無いADRの参照があれば直す。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- Claude Codeのhookの変更。`.claude/settings.json` は `adr-guard claude-hook --allow-status` のままとする（[ADR 0016](0016-allow-status-in-claude-hook.md)）。
- CIでの `adr-guard unimplemented` の実行。一覧は手元で使うものとし、未実装のADRがあってもCIは失敗させない。
- 既存のADRのステータスの書きかえ。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.3のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.3-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0018](0018-update-adr-guard-to-v0.8.2.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.8.3）で、次を確かめる。
  - `adr-guard check-status` と `adr-guard links` が終了コード0で終わる。
  - `adr-guard unimplemented` が終了コード0で終わる。
  - 各コミットでpre-commit hookが許可する。
  - `adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。

## トレードオフ

- `採択` と `採択（未実装）` の使い分けが要る。実装を伴わない採択のADRと、実装待ちのADRを区別できる代わりに、実装を終えたときにステータスの行を `採択` に変える手間が増える。ステータスの行の変更は `--allow-status` で許可されている。
- 手元の adr-guard が v0.8.3 より古いと、`adr-guard unimplemented` が使えない。READMEのインストール手順のタグを上げて補う。

## 参考文献

- tett23/adr-guard [ADR 0016: 採択（未実装）のステータス](https://github.com/tett23/adr-guard/blob/main/docs/adr/0016-accepted-not-implemented-status.md)
- tett23/adr-guard [リリース v0.8.3](https://github.com/tett23/adr-guard/releases/tag/v0.8.3)
