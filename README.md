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
tag=v0.1.1
target=aarch64-apple-darwin
archive="send-to-kindle-$tag-$target.tar.gz"
curl -fsSLO "https://github.com/tett23/send-to-kindle/releases/download/$tag/$archive"
curl -fsSLO "https://github.com/tett23/send-to-kindle/releases/download/$tag/SHA256SUMS"
```

#### `gh` でダウンロードする

```sh
tag=v0.1.1
target=aarch64-apple-darwin
archive="send-to-kindle-$tag-$target.tar.gz"
gh release download "$tag" --repo tett23/send-to-kindle --pattern "$archive" --pattern SHA256SUMS
```

#### 展開して配置する

チェックサムを確かめてから展開し、`PATH` の通ったディレクトリに置く。
アーカイブには設定の例 `.env.example` も入っているので、設定ファイルの置き場所へコピーして値を書きかえる（v0.1.0 のアーカイブには入っていない）。

```sh
grep "$archive" SHA256SUMS | shasum -a 256 -c   # Linuxでは sha256sum -c
tar -xzf "$archive"
mv "${archive%.tar.gz}/send-to-kindle" ~/.local/bin/
mkdir -p ~/.config/send-to-kindle
cp "${archive%.tar.gz}/.env.example" ~/.config/send-to-kindle/.env
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

設定の例は [.env.example](.env.example) にある（リリースのアーカイブにも入っている）。コピーして値を書きかえる。

```sh
mkdir -p ~/.config/send-to-kindle
cp .env.example ~/.config/send-to-kindle/.env
send-to-kindle --env-file ~/.config/send-to-kindle/.env book.epub
```

オプションや設定の一覧は `send-to-kindle --help` で表示できる。
件名「変換」で送るため、KindleはファイルをKindleの形式に変換する。
詳しい挙動は [docs/specifications.md](docs/specifications.md) を参照。

### GmailのSMTPを使う

Gmailから送る場合は、Googleアカウントのパスワードではなく「アプリ パスワード」を使う。

1. Googleアカウントで2段階認証プロセスを有効にする。アプリ パスワードは、2段階認証プロセスを有効にしたアカウントでしか作れない。
2. [アプリ パスワード](https://myaccount.google.com/apppasswords)のページを開き、アプリ名（例：`send-to-kindle`）を入力して作成する。
3. 表示された16文字のパスワードを `SMTP_PASSWORD` に設定する。パスワードは作成時にしか表示されないので、すぐに控える。表示に含まれる空白は取り除く。

詳しくはGoogleのヘルプ「[アプリ パスワードでログインする](https://support.google.com/accounts/answer/185833)」を参照。
職場や学校のアカウント（Google Workspace）では、管理者がアプリ パスワードを無効にしていると作成できない。

設定は次のようにする。

```sh
EMAIL=you@gmail.com
SEND_TO_KINDLE_EMAIL=you@kindle.com
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER_NAME=you@gmail.com
SMTP_PASSWORD=abcdefghijklmnop
```

`EMAIL` のGmailアドレスは、Amazonの「コンテンツと端末の管理」→「設定」→「パーソナル・ドキュメント設定」で、承認済みEメールアドレスに追加しておく。

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
