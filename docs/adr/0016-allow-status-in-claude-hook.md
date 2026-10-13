# ADR 0016: Claude Codeのhookでも、ADRのステータスの行の変更を許可する

ステータス: 採択

## 文脈

[ADR 0009](0009-use-external-adr-guard.md) で、Claude CodeのPreToolUse hook（`.claude/settings.json`）のコマンドを `adr-guard claude-hook` にした。

adr-guard v0.5.0 から、ステータスの行の変更も `--allow-status` を付けたときだけ許可される（tett23/adr-guard の [ADR 0010](https://github.com/tett23/adr-guard/blob/main/docs/adr/0010-allow-status-option.md)）。
[ADR 0014](0014-update-adr-guard-to-v0.5.0.md) では、pre-commit hookとCIに `--allow-status` を付けたが、Claude Codeのhookは全体の設定（`~/.claude/settings.json`）で管理するとして変えなかった。
[ADR 0015](0015-update-adr-guard-to-v0.8.0.md) でも変えていない。

しかし、このリポジトリの `.claude/settings.json` には `adr-guard claude-hook`（`--allow-status` なし）が残っている。
プロジェクトの設定のhookと全体の設定のhookはどちらも実行されるため、Claude CodeがADRを採択して古いADRのステータスの行を書きかえる編集は、このリポジトリのhookで拒否される。
READMEは、Claude Codeのhookは `adr-guard claude-hook --allow-status` で許可されると書いており、実際の設定と食い違っている。

## 実装すること

- `.claude/settings.json` のPreToolUse hookのコマンドを `adr-guard claude-hook --allow-status` にする。
- `docs/specifications.md` のClaude Codeのhookの記述を `adr-guard claude-hook --allow-status` にし、ステータスの行の変更を許可する箇所にClaude Codeのhookを加える。
- [ADR 0009](0009-use-external-adr-guard.md) のステータスを `採択→actionの指定方法は[ADR 0011](0011-pin-adr-guard-action-by-version.md)、Claude Codeのhookのコマンドは[ADR 0016](0016-allow-status-in-claude-hook.md)` にする。

## 実装しないこと

- このリポジトリの `.claude/settings.json` からhookを消して、全体の設定だけに任せること。全体の設定を持たない環境でも、Claude Codeの編集を止められるようにしておく。
- 全体の設定（`~/.claude/settings.json`）の変更。すでに `adr-guard claude-hook --allow-status` になっている。
- `claude-hook` への `--allow-links` の指定。adr-guard v0.8.0 の `claude-hook` は `--allow-links` を受けつけず、付けると引数の誤り（終了コード64）になって、hookが編集を止めなくなる。

## テスト設計

設定の値の変更なので、自動テストは書かない。次の手順で確かめる。

- `adr-guard claude-hook --allow-status` に、コミット済みのADRのステータスの行だけを変えるEditのJSONを標準入力で与えると、終了コード0で許可される。
- 同じコマンドに、コミット済みのADRの本文を変えるEditのJSONを与えると、終了コード2で拒否される。
- `.claude/settings.json` がJSONとして読める。
- 変更のプルリクエストのCIで、`adr` ジョブが成功する。

## トレードオフ

- Claude Codeが古いADRのステータスの行を書きかえられるようになる。ステータスの行以外の変更は、これまでどおり拒否される。
- 同じhookがプロジェクトの設定と全体の設定の両方にあり、二重に実行される。どちらも同じ判定なので、結果は変わらない。
