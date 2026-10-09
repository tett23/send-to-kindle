# ADR 0006: vで始まるタグでリリースを作成する

ステータス: 採択

## 文脈

現在、send-to-kindle を使うには、Rustのツールチェインを入れて `cargo install --path .` でビルドする必要がある。
ADR 0002 ではビルド済みバイナリの配布を、ADR 0004 ではリリースの自動化を、それぞれ「実装しないこと」とした。

Rustのツールチェインが無い環境でも使えるよう、ビルド済みのバイナリをGitHubのリリースで配布したい。

配布するmacOS向けのバイナリには、Apple Developer IDによる署名と公証（notarization）を行わない。
Developer Programへの登録が必要で、個人のツールには見合わないためである。
そのため、ブラウザでダウンロードしたバイナリには検疫属性（`com.apple.quarantine`）が付き、Gatekeeperが実行を止める。
利用者が自分で検疫を外す方法を、READMEに記載する必要がある。

## 実装すること

### ワークフロー

- `.github/workflows/release.yml` に置く。起動条件は、`v` で始まるタグのpushとする。
- 次のジョブを順に実行する。
  1. `verify`：タグがパッケージのバージョンと一致することを確かめる。タグ `v<version>` の `<version>` が `Cargo.toml` の `version` と異なれば失敗する。あわせて `cargo test --locked` を実行する。
  2. `build`：各ターゲットのバイナリをビルドして、アーカイブにまとめる。
  3. `release`：GitHubのリリースを作成し、アーカイブとチェックサムを添付する。
- `permissions` は、ワークフロー全体では `contents: read` とし、`release` ジョブだけ `contents: write` とする。
- 使うアクションは `actions/checkout`、`actions/upload-artifact`、`actions/download-artifact` だけとし、コミットのSHAで固定する。リリースの作成は、ランナーに入っている `gh` コマンドで行う。
- ツールチェインはCI（ADR 0004）と同じく、ランナーに入っている `rustup` でstableを入れる。

### ターゲット

| ターゲット | ランナー |
|---|---|
| `aarch64-apple-darwin` | `macos-latest` |
| `x86_64-apple-darwin` | `macos-latest`（クロスコンパイル） |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` |

- `cargo build --release --locked --target <ターゲット>` でビルドする。

### 成果物

- ターゲットごとに `send-to-kindle-<タグ>-<ターゲット>.tar.gz` を作る。中身はバイナリ `send-to-kindle`、`LICENSE`、`README.md` とする。
- すべてのアーカイブのSHA-256を `SHA256SUMS` にまとめ、リリースに添付する。
- リリースのタイトルはタグ名とし、本文はGitHubの自動生成（`gh release create --generate-notes`）とする。

### 署名

- macOS向けのバイナリには、Apple Developer IDによる署名と公証を行わない。
- Apple Silicon向けのバイナリには、リンカーが付けるアドホック署名だけが付く（Apple Siliconでは署名の無いバイナリは実行できないため、これは必須である）。

### ドキュメント

- `README.md` に次を記載する。
  - リリースからのインストール手順（ダウンロード、展開、`PATH` の通った場所への配置）
  - `SHA256SUMS` によるチェックサムの確かめ方
  - macOSのバイナリが署名・公証されていないこと
  - 検疫の外し方：`xattr -d com.apple.quarantine <パス>`。あわせて、ブラウザでダウンロードした場合に検疫属性が付き、`curl` などでダウンロードした場合は付かないこと
  - 検疫を外すことは、そのバイナリを信頼するという判断であり、チェックサムを確かめてから行うこと
- `docs/specifications.md` に、リリースの手順（`Cargo.toml` の `version` を更新してコミットし、`v<version>` のタグをpushする）と成果物を記載する。
- ADR 0002 のステータスを `採択→0003、0005、0006` に、ADR 0004 のステータスを `採択→0006` にする。

## 実装しないこと

- Apple Developer IDによる署名と公証。
- Windows向けと、`aarch64-unknown-linux-gnu` などその他のターゲットのバイナリ。
- `musl` による静的リンクのLinux向けバイナリ。
- Homebrewのformula、crates.ioへの公開。
- バージョンの自動更新や、タグの自動作成。
- `v` で始まらないタグでのリリース作成。
- 成果物への署名（GPGやSigstoreなど）。チェックサムのみとする。

## テスト設計

- タグとバージョンの照合は、ワークフローのシェルで行う。照合の処理をRustのテストにするほどの複雑さは無いため、次を実際の実行で確かめる。
  - バージョンと一致するタグ（例：`v0.1.0`）をpushすると、リリースが作成され、3つのアーカイブと `SHA256SUMS` が添付される。
  - 添付された `SHA256SUMS` で、ダウンロードしたアーカイブを `shasum -a 256 -c` で検証できる。
  - macOS（Apple Silicon）で、ブラウザでダウンロードしたバイナリが検疫で止められ、READMEの手順で検疫を外すと `send-to-kindle --help` が実行できる。
- 一致しないタグでの失敗は、フォークや別リポジトリを使わずに確かめる手段が無いため、テストしない。照合の処理は短く、レビューで確認する。

## トレードオフ

- 署名と公証を行わないため、利用者は検疫を自分で外す必要があり、手間がかかる。また、検疫を外す操作は、利用者にバイナリの信頼を委ねる。チェックサムの確かめ方を併記して補う。
- ターゲットを3つに絞るため、それ以外の環境では従来どおり `cargo install` が必要になる。
- `x86_64-apple-darwin` はApple Siliconのランナーでのクロスコンパイルとするため、ランナー上ではテストを実行しない。依存はRustのみで書かれており（`lettre` と rustls）、クロスコンパイルの問題は起きにくい。
- リリースの本文を自動生成にするため、内容はプルリクエストのタイトルに依存する。変更履歴（CHANGELOG）は書かない。
