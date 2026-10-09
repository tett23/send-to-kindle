# send-to-kindle

[![CI](https://github.com/tett23/send-to-kindle/actions/workflows/ci.yml/badge.svg)](https://github.com/tett23/send-to-kindle/actions/workflows/ci.yml)

ファイルをKindleのメールアドレスへ添付して送るコマンド。

## インストール

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
コミット済みのADRはステータスの行を除いて変更できず、pre-commit hookとClaude Codeのhookで強制される。
hookの実行にはRustツールチェイン（`cargo`）が必要。

## License

[MIT](LICENSE)
