# send-to-kindle

## 開発

クローン後、gitのhookを有効化する。

```sh
git config core.hooksPath .githooks
```

設計判断は `docs/adr/` にADRとして記録する。
コミット済みのADRはステータスの行を除いて変更できず、pre-commit hookとClaude Codeのhookで強制される。
hookの実行にはRustツールチェイン（`cargo`）が必要。
仕様の全体像は [docs/specifications.md](docs/specifications.md) を参照。

## License

[MIT](LICENSE)
