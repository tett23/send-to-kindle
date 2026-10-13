# ADR 0014: adr-guard を v0.5.0 に上げ、ステータスの行の変更を `--allow-status` で許可する

ステータス: 採択→指定するバージョンは[ADR 0015](0015-update-adr-guard-to-v0.8.0.md)

## 文脈

[ADR 0013](0013-update-adr-guard-to-v0.4.0.md) で、CIの `adr` ジョブの adr-guard のactionを v0.4.0 とし、pre-commit hookとactionに `--allow-links`（`allow-links: true`）を付けた。

その後、tett23/adr-guard は v0.5.0 に更新された（tett23/adr-guard の [ADR 0010](https://github.com/tett23/adr-guard/blob/main/docs/adr/0010-allow-status-option.md)）。
v0.5.0 では、コミット済みのADRのステータスの行の変更も、`--allow-status` を付けたときだけ許可する。
付けないと、ADRを採択したときに古いADRのステータスを `採択→...` に変えることもできない。

手元の adr-guard はすでに v0.5.0 に更新されている。
このリポジトリのpre-commit hookは `--allow-links` だけを付けているため、いまはステータスの行を変えるコミットがpre-commit hookで止まる。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.5.0` にし、入力 `allow-status: true` を加える（`allow-links: true` はそのまま）。
- `.githooks/pre-commit` を `exec adr-guard pre-commit --allow-status --allow-links` にする。
  - このADRの実装のコミットでは、[ADR 0013](0013-update-adr-guard-to-v0.4.0.md) のステータスの行を変えるため、hookの変更を同じコミットか、それより前のコミットに含める。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.5.0 にし、ステータスの行の変更とリンクだけの変更が、pre-commit hookとCIの `--allow-status` と `--allow-links` で許可されることを記載する。
- `docs/specifications.md` のADRとCIの記述を v0.5.0 に合わせる（`--allow-status`）。
- [ADR 0013](0013-update-adr-guard-to-v0.4.0.md) のステータスを `採択→指定するバージョンとステータスの行の許可は[ADR 0014](0014-update-adr-guard-to-v0.5.0.md)` にする。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- Claude Codeのhookの変更。Claude Codeのhookは全体の設定（`~/.claude/settings.json`）にあり、このリポジトリでは管理しない。全体の設定は `adr-guard claude-hook --allow-status` に更新済み（反映は各自の環境で行う）。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.5.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.5.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links` と `links` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0013](0013-update-adr-guard-to-v0.4.0.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.5.0）で、次を確かめる（コミットはしない）。
  - コミット済みのADRのステータスの行だけを変えると、pre-commit hookが許可する。
  - コミット済みのADRの本文を変えると、pre-commit hookが拒否する。

## トレードオフ

- pre-commit hookに `--allow-status` を付けるため、手元の adr-guard が v0.5.0 より古いと、引数の誤り（終了コード64）でコミットが止まる。READMEのインストール手順のタグを上げて補う。
- ステータスの行の変更を許可するかどうかが、hookとactionの設定から読み取れるようになる。その分、設定に付けるオプションが増える。
