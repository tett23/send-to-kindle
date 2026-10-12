# ADR 0012: adr-guard を v0.3.0 に上げる

ステータス: 採択

## 文脈

[ADR 0011](0011-pin-adr-guard-action-by-version.md) で、CIの `adr` ジョブの adr-guard のactionをバージョンのタグで指定し、バージョンを当時の最新の v0.2.1 とした。

その後、tett23/adr-guard は v0.3.0 まで更新された。
v0.3.0 では、意図してADRを書きかえるコミットを、コミットメッセージのトレーラー `ADR-Guard-Allow: <理由>`（CI）と環境変数 `ADR_GUARD_ALLOW`（pre-commit hook）で許可できるようになった（tett23/adr-guard の [ADR 0008](https://github.com/tett23/adr-guard/blob/main/docs/adr/0008-allow-intentional-adr-rewrites.md)）。
ほかに、ADRを一覧する `adr-guard list` と `adr-guard proposed` も加わった。

このリポジトリでも、ADRの参照をリンクにするためにADRの本文を書きかえたコミット（`655de1b` のマージ）で、CIの `adr` ジョブが失敗した。
v0.2.1 のactionには、意図した書きかえをCIで通す手段がない。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.3.0` にする。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.3.0 にする。
- `README.md` の開発の節に、意図してADRを書きかえる場合の手順（`ADR_GUARD_ALLOW` でコミットし、コミットメッセージに `ADR-Guard-Allow` のトレーラーを付ける）を、adr-guard のREADMEへの参照とともに加える。
- `docs/specifications.md` のCIの記述（「現在は `v0.2.1`」）を v0.3.0 にし、意図した書きかえをトレーラーで許可できることを記載する。
- [ADR 0011](0011-pin-adr-guard-action-by-version.md) のステータスを `採択→指定するバージョンはADR 0012` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- 過去のCIの失敗（`655de1b`）への対応。範囲から外れているため、今後のCIには影響しない。
- adr-guard の更新を自動で取りこむ仕組み。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.3.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.3.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証されていることを確かめる。
- 手元の adr-guard を v0.3.0 にしたうえで、コミット済みのADRの本文の変更が、pre-commit hookで拒否され、`ADR_GUARD_ALLOW` を付けると許可されることを確かめる（コミットはしない）。

## トレードオフ

- トレーラーを付ければ、CIでADRの書きかえを通せるようになる。意図しない書きかえを防ぎ、意図した書きかえを理由とともに履歴に残すことを優先する。トレーラーはプルリクエストのレビューで確かめる。
- 手元の adr-guard が v0.3.0 より古いと、`ADR_GUARD_ALLOW` が効かない。READMEのインストール手順のタグを上げて補う。
