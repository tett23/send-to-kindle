# ADR 0015: adr-guard を v0.8.0 に上げ、ADRのステータスの行を検査する

ステータス: 採択→指定するバージョンは[ADR 0017](0017-update-adr-guard-to-v0.8.1.md)

## 文脈

[ADR 0014](0014-update-adr-guard-to-v0.5.0.md) で、CIの `adr` ジョブの adr-guard のactionを v0.5.0 とし、pre-commit hookとactionに `--allow-status`（`allow-status: true`）を付けた。

その後、tett23/adr-guard は v0.8.0 まで更新された。

- v0.6.0：どの深さの `docs/adr/`・`docs/<モジュール>/adr/` も保護の対象になった（tett23/adr-guard の [ADR 0011](https://github.com/tett23/adr-guard/blob/main/docs/adr/0011-nested-adr-directories.md)）。
- v0.7.0：ADRは、行頭が `ステータス:` か `Status:` のステータスの行をちょうど1つ持たなければならない（tett23/adr-guard の [ADR 0012](https://github.com/tett23/adr-guard/blob/main/docs/adr/0012-status-line-and-status-adt.md)）。無い、または2つ以上あると、pre-commit hookと新しいサブコマンド `adr-guard check-status` で違反になる。actionは既定で `check-status` も実行する（入力 `check-status: false` で止められる）。`--allow-status` で、ステータスの行が無かったADRへの1行の追加も許可される。
- v0.8.0：`docs/audit/` と `docs/benchmarks/`（どの深さの `docs/` でも）のコミット済みのファイルも、変更・削除・移動できない（tett23/adr-guard の [ADR 0013](https://github.com/tett23/adr-guard/blob/main/docs/adr/0013-protect-audit-and-benchmark-records.md)）。`--allow-status` と `--allow-links` は記録には効かない。

手元の adr-guard はすでに v0.8.0 に更新されている。
このリポジトリのADR（0001〜0014）は、すべて `ステータス:` の行をちょうど1つ持っており、`adr-guard check-status` は終了コード0で終わる。そのため、ステータスの行の移行は要らない。
このリポジトリには `docs/audit/` と `docs/benchmarks/` は無い。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.0` にする。入力 `allow-status: true` と `allow-links: true` はそのままとし、`check-status: false` は付けない（actionの既定で `check-status` を実行する）。
  - actionの上のADRの番号を並べたコメントに 0015 を加える。
- `.githooks/pre-commit` は `exec adr-guard pre-commit --allow-status --allow-links` のままとし、コメントのADRの番号に 0015 を加える。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.8.0 にし、次を記載する。
  - ADRには `ステータス:` で始まる行がちょうど1つ要り、pre-commit hookとCIで検査されること。`adr-guard check-status` で確かめられること。
  - コミット済みの `docs/audit/` と `docs/benchmarks/` のファイルも変更・削除・移動できないこと。
- `docs/specifications.md` のADRとCIの記述を v0.8.0 に合わせる（ステータスの行の検査、`check-status`、記録の保護）。
- [ADR 0014](0014-update-adr-guard-to-v0.5.0.md) のステータスを `採択→指定するバージョンは[ADR 0015](0015-update-adr-guard-to-v0.8.0.md)` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- 既存のADRのステータスの行の移行。すでに全ADRが条件を満たしている。
- `docs/audit/` と `docs/benchmarks/` の作成。記録が要るときに作る。
- Claude Codeのhookの変更。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0014](0014-update-adr-guard-to-v0.5.0.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.8.0）で、次を確かめる（コミットはしない）。
  - `adr-guard check-status` と `adr-guard links` が終了コード0で終わる。
  - 変更をステージすると、pre-commit hookが許可する。
  - コミット済みのADRの本文を変えると、pre-commit hookが拒否する。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。

## トレードオフ

- 手元の adr-guard が v0.7.0 より古いと、ステータスの行の検査が手元では行われず、CIで初めて違反に気づく。READMEのインストール手順のタグを上げて補う。
- ステータスの行の書き方（`- ステータス:` など）が固定される。その分、ADRのステータスを機械的に一覧・検査できる。
- 記録（`docs/audit/`、`docs/benchmarks/`）は誤記も直せなくなる。直す場合は新しい記録を作成する。
