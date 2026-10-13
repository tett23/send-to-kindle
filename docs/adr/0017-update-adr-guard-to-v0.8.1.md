# ADR 0017: adr-guard を v0.8.1 に上げ、ADRと記録のディレクトリの別名も保護する

ステータス: 採択

## 文脈

[ADR 0015](0015-update-adr-guard-to-v0.8.0.md) で、CIの `adr` ジョブの adr-guard のactionを v0.8.0 とした。

その後、tett23/adr-guard は v0.8.1 に更新された（tett23/adr-guard の [ADR 0014](https://github.com/tett23/adr-guard/blob/main/docs/adr/0014-directory-name-variants.md)、[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.1)）。

- ADRのディレクトリとして、`adr` のほか `adrs`、`decisions`、`decision` も保護する。
- 記録のディレクトリとして、`docs/audit/`・`docs/benchmarks/` のほか `docs/audits/`・`docs/benchmark/` も保護する。

手元の adr-guard はすでに v0.8.1 に更新されている。
このリポジトリには、新しく保護される名前（`adrs`、`decisions`、`decision`、`audits`、`benchmark`）のディレクトリは無い（追跡しているファイル、追跡していないディレクトリとも）。そのため、ディレクトリの名前の変更は要らない。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.1` にする。入力 `allow-status: true` と `allow-links: true` はそのままとする。
  - actionの上のADRの番号を並べたコメントに 0017 を加える。
- `.githooks/pre-commit` は `exec adr-guard pre-commit --allow-status --allow-links` のままとし、コメントのADRの番号に 0017 を加える。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.8.1 にし、保護される記録のディレクトリに `docs/audits/` と `docs/benchmark/` を加える。
- `docs/specifications.md` の記録の保護とCIの記述を v0.8.1 に合わせる。
- [ADR 0015](0015-update-adr-guard-to-v0.8.0.md) のステータスを `採択→指定するバージョンは[ADR 0017](0017-update-adr-guard-to-v0.8.1.md)` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- ADRのディレクトリの変更。このリポジトリのADRは引き続き `docs/adr/` に置く。
- 記録のディレクトリの作成。記録が要るときに作る。
- Claude Codeのhookの変更。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.1のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.1-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0015](0015-update-adr-guard-to-v0.8.0.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.8.1）で、次を確かめる（コミットはしない）。
  - `adr-guard check-status` と `adr-guard links` が終了コード0で終わる。
  - 変更をステージすると、pre-commit hookが許可する。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。

## トレードオフ

- 今後 `docs/audits/`、`docs/benchmark/` や `adrs` などの名前のディレクトリを作ると、コミット後は変更できなくなる。ADRや記録以外の用途にはこれらの名前を使わない。
- 手元の adr-guard が v0.8.1 より古いと、別名のディレクトリの保護が手元では行われず、CIで初めて違反に気づく。READMEのインストール手順のタグを上げて補う。
