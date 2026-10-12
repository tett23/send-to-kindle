# ADR 0011: adr-guard のactionをバージョンのタグで指定する

ステータス: 採択

## 文脈

[ADR 0009](0009-use-external-adr-guard.md) で、CIの `adr` ジョブを tett23/adr-guard のcomposite actionに置きかえ、actionをコミットのSHA（v0.1.0 のコミット）で固定した。

SHAでの指定は、どのバージョンを使っているかがコメントを見ないとわからず、更新のたびにタグからSHAを調べ直す手間がかかる。
tett23/adr-guard は自分たちで管理しているリポジトリであり、タグが書きかえられる心配は小さい。
また、tett23/adr-guard のREADMEも、actionをバージョンのタグで指定する例を示している。

あわせて、tett23/adr-guard は v0.1.0 のあと v0.2.1 まで更新されている（モジュールごとのADRディレクトリの保護、`--version` と `--help` の追加）。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブで、actionを `uses: tett23/adr-guard@v0.2.1` のようにバージョンのタグで指定する。
- 指定するバージョンは、現時点の最新の v0.2.1 とする。
- `README.md` の開発の節にある adr-guard のインストール手順のタグも v0.2.1 にする。
- `docs/specifications.md` のCIの記述（「コミットのSHAで固定」）を、バージョンのタグで指定する形に更新する。
- [ADR 0009](0009-use-external-adr-guard.md) のステータスを `採択→actionの指定方法はADR 0011` にする。

## 実装しないこと

- ほかのactions（`actions/checkout`、`actions/cache` など）の指定方法の変更。これらは外部のリポジトリなので、コミットのSHAでの固定を続ける。
- adr-guard の更新を自動で取りこむ仕組み（Dependabotなど）。
- `v0` のようなメジャーバージョンだけのタグでの指定。tett23/adr-guard はそのようなタグを作っていない。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.2.1のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.2.1-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証されていることを確かめる。
- 手元の adr-guard を v0.2.1 にしたうえで、コミット済みのADRの本文の変更がpre-commit hookで拒否されることを確かめる。

## トレードオフ

- タグはコミットのSHAと違い、書きかえられる可能性がある。tett23/adr-guard は自分たちで管理しており、リリースのタグを書きかえない運用のため、読みやすさと更新のしやすさを優先した。
- actionはタグの指すコミットのcomposite actionを使い、ダウンロードするバイナリはactionの `Cargo.toml` のバージョンのリリースになる。リリースのアーカイブは `SHA256SUMS` で検証されるが、`SHA256SUMS` 自体も同じリリースから取得するため、リリースが差しかえられた場合は検知できない。これはSHAで固定していた場合も同じである。
