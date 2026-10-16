# ADR 0018: adr-guard を v0.8.2 に上げ、コミット済みのADRのリンクだけの変更を許可する

ステータス: 採択

## 文脈

[ADR 0017](0017-update-adr-guard-to-v0.8.1.md) で、CIの `adr` ジョブの adr-guard のactionを v0.8.1 とした。

その後、tett23/adr-guard は v0.8.2 に更新された（tett23/adr-guard の [ADR 0015](https://github.com/tett23/adr-guard/blob/main/docs/adr/0015-allow-link-changes.md)、[リリース](https://github.com/tett23/adr-guard/releases/tag/v0.8.2)）。

- `--allow-links` が、ADRの参照のリンクを付ける変更に限らず、コミット済みのADRの「リンクだけの変更」全般を許可するようになった。インラインリンク・参照形式リンクを付ける・外す・リンク先を変える変更、画像のリンク先の変更、リンクの参照定義の行（`[1]: URL`）の変更が通る。
- リンクのテキストの変更、画像の追加・削除、自動リンク（`<URL>`）と本文のURLの変更は、引き続き拒否される。
- 監査や計測の記録は、引き続き `--allow-links` に関わらず完全に保護される。

このリポジトリはすでに pre-commit hookとCIで `--allow-links` を付けている（[ADR 0013](0013-update-adr-guard-to-v0.4.0.md)）。そのため、バージョンを上げるだけで、許可される変更が広がる。
手元の adr-guard はすでに v0.8.2 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.8.2` にする。入力 `allow-status: true` と `allow-links: true` はそのままとする。
  - actionの上のADRの番号を並べたコメントに 0018 を加える。
- `.githooks/pre-commit` は `exec adr-guard pre-commit --allow-status --allow-links` のままとし、コメントのADRの番号に 0018 を加える。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.8.2 にし、`--allow-links` の説明を「リンクだけの変更」に合わせる。
- `docs/specifications.md` の `--allow-links` とCIの記述を v0.8.2 に合わせる。
- [ADR 0017](0017-update-adr-guard-to-v0.8.1.md) のステータスを `採択→指定するバージョンは[ADR 0018](0018-update-adr-guard-to-v0.8.2.md)` にする。
- `adr-guard links --fix --include-committed` を実行し、リンクの無いADRの参照があれば直す。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- Claude Codeのhookの変更。`.claude/settings.json` は `adr-guard claude-hook --allow-status` のままとする（[ADR 0016](0016-allow-status-in-claude-hook.md)）。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.8.2のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.8.2-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0017](0017-update-adr-guard-to-v0.8.1.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.8.2）で、次を確かめる。
  - `adr-guard check-status` と `adr-guard links` が終了コード0で終わる。
  - 各コミットでpre-commit hookが許可する。
  - `adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。

## トレードオフ

- コミット済みのADRのリンク先を変えられるようになる。リンク先の変更でADRの意味が変わりうるが、レビューで確かめる。リンクのテキストや本文は引き続き変更できない。
- 手元の adr-guard が v0.8.2 より古いと、手元のpre-commit hookはリンクだけの変更の一部を拒否する。READMEのインストール手順のタグを上げて補う。

## 参考文献

- tett23/adr-guard [ADR 0015: allow link changes](https://github.com/tett23/adr-guard/blob/main/docs/adr/0015-allow-link-changes.md)
- tett23/adr-guard [リリース v0.8.2](https://github.com/tett23/adr-guard/releases/tag/v0.8.2)
