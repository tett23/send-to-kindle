# ADR 0020: adr-guard を v0.10.0 に上げ、ADRの番号の重複と修飾子付きの参照を検査する

ステータス: 採択

## 文脈

[ADR 0019](0019-update-adr-guard-to-v0.8.3.md) で、CIの `adr` ジョブの adr-guard のactionを v0.8.3 とした。

その後、tett23/adr-guard は v0.9.0、v0.9.1、v0.10.0 に更新された（[リリース v0.9.0](https://github.com/tett23/adr-guard/releases/tag/v0.9.0)、[リリース v0.9.1](https://github.com/tett23/adr-guard/releases/tag/v0.9.1)、[リリース v0.10.0](https://github.com/tett23/adr-guard/releases/tag/v0.10.0)）。

- v0.9.0：新しいサブコマンド `adr-guard check-numbers` が加わった。同じディレクトリで番号が重複したADRを報告する。actionは既定でこれを実行し、重複があればCIが失敗する。重複を直す `adr-guard renumber [--dry-run]` と、その振りなおしを許可する `--allow-renumber`（`pre-commit` と `check-range`、actionの入力 `allow-renumber`。既定は無効）も加わった（tett23/adr-guard の [ADR 0018: 重複したADRの番号の修復](https://github.com/tett23/adr-guard/blob/main/docs/adr/0018-repair-duplicate-adr-numbers.md)）。
- v0.9.1：`renumber` の誤報が直された。
- v0.10.0：`ADR NNNN` の直前の語（空白1つ）を修飾子として扱う。`/` を含む語はディレクトリ（`docs/adr ADR 0001`）、`-` を含む語はモジュール（`docs/<モジュール>/adr/`）を指す。それ以外の語は、そのモジュールが存在するときだけ修飾子とする。修飾子はリンクのテキストに含める。`links` とpre-commit hookは、参照先の無い修飾子付きの参照、リンク先が修飾子のADRと異なるリンク、リンクの外に出た修飾子を報告する。`links --fix` で直せるものは直す。また、シンボリックリンクの `.md` は検査しない（tett23/adr-guard の [ADR 0019: モジュールで修飾したADRの参照](https://github.com/tett23/adr-guard/blob/main/docs/adr/0019-module-qualified-adr-references.md)、[ADR 0020: シンボリックリンクの文書を飛ばす](https://github.com/tett23/adr-guard/blob/main/docs/adr/0020-skip-symlinked-documents.md)）。

このリポジトリのADRは `docs/adr/` だけにあり、モジュールごとのADRは無い。
手元の adr-guard（v0.10.0）で `adr-guard links --fix --include-committed` を実行しても、書きかわるファイルは無かった。`adr-guard check-numbers` も重複を報告しない。
手元の adr-guard はすでに v0.10.0 に更新されている。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.10.0` にする。入力 `allow-status: true` と `allow-links: true` はそのままとし、`check-numbers` と `allow-renumber` は指定しない（既定のとおり `check-numbers` を実行し、振りなおしは許可しない）。
  - actionの上のADRの番号を並べたコメントに 0020 を加える。
- `.githooks/pre-commit` は `exec adr-guard pre-commit --allow-status --allow-links` のままとし、コメントのADRの番号に 0020 を加える。
- `README.md` の開発の節にある adr-guard のインストール手順のタグを v0.10.0 にし、ADRの番号の重複がCIでエラーになること（`adr-guard check-numbers`）と、ほかのリポジトリのADRへの参照の書き方を書く。
- `docs/specifications.md` のCIの記述を v0.10.0 に合わせ、`adr` ジョブが `check-numbers` も実行することと、修飾子付きの参照の規則を書く。
- [ADR 0019](0019-update-adr-guard-to-v0.8.3.md) のステータスを `採択→指定するバージョンは[ADR 0020](0020-update-adr-guard-to-v0.10.0.md)` にする。
- `adr-guard links --fix --include-committed` を実行し、直すべき参照があれば直す。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- Claude Codeのhookの変更。`.claude/settings.json` は `adr-guard claude-hook --allow-status` のままとする（[ADR 0016](0016-allow-status-in-claude-hook.md)）。
- `--allow-renumber` の常時の許可。番号の重複が起きたときは、その修復のプルリクエストで一時的に付ける。
- モジュールごとのADRのディレクトリの導入。
- 手元の adr-guard の更新の自動化。

## テスト設計

- 変更のプルリクエストのCIで、`adr` ジョブがv0.10.0のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.10.0-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status`、`check-numbers` が実行されていることを確かめる。
  - このプルリクエストには [ADR 0019](0019-update-adr-guard-to-v0.8.3.md) のステータスの行の変更が含まれるため、`--allow-status` が効いていることも確かめられる。
- 手元の adr-guard（v0.10.0）で、次を確かめる。
  - `adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` が終了コード0で終わる。
  - 各コミットでpre-commit hookが許可する。
  - `adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。

## トレードオフ

- 別のブランチで同じ番号のADRを足すと、マージ後の `main` でCIが失敗するようになる。重複に早く気づける代わりに、`adr-guard renumber` での修復と `--allow-renumber` を付けたコミットが要る。
- 修飾子の規則により、`ADR NNNN` の直前の語によっては参照先が `docs/adr/` 以外と解釈される。ほかのリポジトリのADRを書くときは、リンクのテキストごと外部リンクにする必要がある。
- 手元の adr-guard が v0.10.0 より古いと、`check-numbers` と修飾子付きの参照の検査が手元で働かず、CIで初めて失敗する。READMEのインストール手順のタグを上げて補う。

## 参考文献

- tett23/adr-guard [ADR 0018: 重複したADRの番号の修復](https://github.com/tett23/adr-guard/blob/main/docs/adr/0018-repair-duplicate-adr-numbers.md)
- tett23/adr-guard [ADR 0019: モジュールで修飾したADRの参照](https://github.com/tett23/adr-guard/blob/main/docs/adr/0019-module-qualified-adr-references.md)
- tett23/adr-guard [ADR 0020: シンボリックリンクの文書を飛ばす](https://github.com/tett23/adr-guard/blob/main/docs/adr/0020-skip-symlinked-documents.md)
- tett23/adr-guard [リリース v0.9.0](https://github.com/tett23/adr-guard/releases/tag/v0.9.0)、[リリース v0.9.1](https://github.com/tett23/adr-guard/releases/tag/v0.9.1)、[リリース v0.10.0](https://github.com/tett23/adr-guard/releases/tag/v0.10.0)
