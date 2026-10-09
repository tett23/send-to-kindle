# send-to-kindle

[![CI](https://github.com/tett23/send-to-kindle/actions/workflows/ci.yml/badge.svg)](https://github.com/tett23/send-to-kindle/actions/workflows/ci.yml)

ファイルをKindleのメールアドレスへ添付して送るコマンド。

## インストール

### リリースから

[Releases](https://github.com/tett23/send-to-kindle/releases) に、環境ごとのアーカイブと、チェックサムをまとめた `SHA256SUMS` を置いている。

| 環境 | ターゲット |
|---|---|
| macOS（Apple Silicon） | `aarch64-apple-darwin` |
| macOS（Intel） | `x86_64-apple-darwin` |
| Linux（x86_64） | `x86_64-unknown-linux-gnu` |

アーカイブ名は `send-to-kindle-<タグ>-<ターゲット>.tar.gz` である。
ダウンロードには `curl`、`gh`、ブラウザのどれを使ってもよい。
macOSでは、`curl` か `gh` を使うと検疫の対処が要らない（後述）。

#### `curl` でダウンロードする

```sh
tag=v0.1.0
target=aarch64-apple-darwin
archive="send-to-kindle-$tag-$target.tar.gz"
curl -fsSLO "https://github.com/tett23/send-to-kindle/releases/download/$tag/$archive"
curl -fsSLO "https://github.com/tett23/send-to-kindle/releases/download/$tag/SHA256SUMS"
```

#### `gh` でダウンロードする

```sh
tag=v0.1.0
target=aarch64-apple-darwin
archive="send-to-kindle-$tag-$target.tar.gz"
gh release download "$tag" --repo tett23/send-to-kindle --pattern "$archive" --pattern SHA256SUMS
```

#### 展開して配置する

チェックサムを確かめてから展開し、`PATH` の通ったディレクトリに置く。

```sh
grep "$archive" SHA256SUMS | shasum -a 256 -c   # Linuxでは sha256sum -c
tar -xzf "$archive"
mv "${archive%.tar.gz}/send-to-kindle" ~/.local/bin/
```

#### 署名と検疫（macOS）

macOS向けのバイナリは、Apple Developer IDによる署名と公証（notarization）をしていない。
Apple Silicon向けのバイナリには、ビルド時に付くアドホック署名だけが付いている。

ブラウザでダウンロードしたファイルには検疫属性（`com.apple.quarantine`）が付き、そのまま実行するとGatekeeperに止められる。
展開したバイナリにも検疫属性は引き継がれる。
一方、`curl` や `gh` でダウンロードしたファイルには検疫属性が付かないため、そのまま実行できる。

ブラウザでダウンロードした場合は、検疫を外す必要がある。
検疫を外すと、そのバイナリを信頼して実行することになる。
上のとおりチェックサムを確かめてから、次のコマンドで外す。

```sh
xattr -d com.apple.quarantine ~/.local/bin/send-to-kindle
```

検疫属性が付いているかは `xattr -p com.apple.quarantine <パス>` で確かめられる。
付いていなければ `No such xattr` と表示されるだけで、外す必要はない。

### ソースから

Rustのツールチェインが必要。

```sh
cargo install --path .
```

## 使い方

```sh
send-to-kindle [--env-file <パス>] <ファイル>
```

設定は `--env-file`（短縮形 `-e`）で指定したファイルから読む。
指定しなければカレントディレクトリの `.env` から、それも無ければ環境変数から読む。

```sh
EMAIL=me@example.com               # 送信元（Kindleの承認済みアドレス）
SEND_TO_KINDLE_EMAIL=me@kindle.com # Kindleのメールアドレス
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER_NAME=user
SMTP_PASSWORD=secret
```

オプションや設定の一覧は `send-to-kindle --help` で表示できる。
件名「変換」で送るため、KindleはファイルをKindleの形式に変換する。
詳しい挙動は [docs/specifications.md](docs/specifications.md) を参照。

## 開発

クローン後、gitのhookを有効化する。

```sh
git config core.hooksPath .githooks
```

テストは `cargo test` で実行する。

設計判断は `docs/adr/` にADRとして記録する。
コミット済みのADRはステータスの行を除いて変更できず、pre-commit hook、Claude Codeのhook、CIで強制される。
hookの実行にはRustツールチェイン（`cargo`）が必要。

### リリース

`Cargo.toml` の `version` を更新してコミットし、`v<version>` のタグをpushする。
GitHub Actionsがバイナリをビルドし、リリースを作成する。

```sh
git tag v0.1.0
git push origin v0.1.0
```

## License

[MIT](LICENSE)
