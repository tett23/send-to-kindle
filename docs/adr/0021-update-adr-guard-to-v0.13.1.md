# ADR 0021: adr-guard を v0.13.1 に上げ、ADRのステータスの行を新形式に移す

Status: accepted

## 文脈

[ADR 0020](0020-update-adr-guard-to-v0.10.0.md) で、CIの `adr` ジョブの adr-guard のactionを v0.10.0 とした。
pre-commit hookは `adr-guard pre-commit --allow-status --allow-links`、Claude Codeのhook（`.claude/settings.json`）は `adr-guard claude-hook --allow-status` を実行している。
ADRのステータスの行は、すべて `ステータス: 採択` のような日本語の形（以下、旧形式）で書いてある。

その後、tett23/adr-guard は v0.11.0、v0.12.0、v0.13.0、v0.13.1 に更新された（[リリース v0.11.0](https://github.com/tett23/adr-guard/releases/tag/v0.11.0)、[リリース v0.12.0](https://github.com/tett23/adr-guard/releases/tag/v0.12.0)、[リリース v0.13.0](https://github.com/tett23/adr-guard/releases/tag/v0.13.0)、[リリース v0.13.1](https://github.com/tett23/adr-guard/releases/tag/v0.13.1)）。

- v0.11.0：Codex のPreToolUse hookとして `adr-guard codex-hook` が加わった（tett23/adr-guard の [ADR 0021: Codex の PreToolUse hook で、コミット済みのADRと記録の編集を拒否する](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0021-codex-hook.md)）。
- v0.12.0：ステータスの行の新しい形（以下、新形式）と、状態遷移の検査が加わった（tett23/adr-guard の [ADR 0022: 状態と遷移を別々の代数的データ型で表す](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0022-status-transitions.md)、[ADR 0023: trace で元の決定から修正ADRをたどる](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0023-trace-amendments.md)）。
  - 新形式は `Status: <トークン>` で、トークンは `proposed`、`accepted`、`accepted-pending`、`rejected`、`discarded`、`deprecated`、または `superseded [<変更の JSON 配列>]` である。変更は `{"part":<変更した部分の名前か null>,"by":[<修正したADRのリポジトリのルートからのパス>]}` の形で書く。
  - 新しいADRは、最初に `Status: proposed` でコミットしなければならない。追加したADRと、ステータスの行を変えたADRには、新形式が要る。旧形式の行は、変えなければそのまま読める。
  - pre-commit と `check-range` は、ステータスの行の状態遷移（`proposed` から `accepted` へ、`accepted` から `superseded` へ、など）を検査する。この検査は `--allow-status`、環境変数 `ADR_GUARD_ALLOW`、トレーラー `ADR-Guard-Allow` では免除されない。
  - 旧形式を新形式に書き換える移行スクリプト `scripts/migrate-adr-status` が adr-guard のソースに加わった。リリースのアーカイブには入っていない。
  - 元の決定から修正したADRをたどる `adr-guard trace` が加わった。
- v0.13.0：移行スクリプトに、書き換えずに差分だけを示す `--dry-run` が加わり、移行できないADRを最後まで数えて `移行できる: N件、移行済み: K件、移行できない: M件` を標準エラーに表示するようになった。新形式に `superseded-pending`、変更の `kind` と `note`、`https://` で始まる修正先、`trace --list` の6列が加わり、旧形式の修正先の読み方が広がった（tett23/adr-guard の [ADR 0024: 移行スクリプトに --dry-run を設け、失敗を最後まで数える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0024-migration-dry-run-and-error-count.md)、[ADR 0025: 新形式のステータス行に、実装待ちの修正、補足、注記、別のリポジトリの修正先を加える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0025-extend-canonical-status-amendments.md)、[ADR 0026: 旧形式のステータス行の修正先を、リンク先と裸の番号からも読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0026-read-more-legacy-references.md)、[ADR 0027: 旧形式の `置換（…）` を、修正を受けた状態として読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0027-read-legacy-replaced-status.md)）。
- v0.13.1：移行スクリプトが作る部分名の不具合が直った。v0.13.0 の移行スクリプトは、部分名に修飾子（`ADR NNNN` の直前の語）、先頭の「。」、末尾の「→」を残すことがあった。v0.13.1 はこれらを除き、部分名を決められない行は「部分名を決められません」と報告して書かない（tett23/adr-guard の [ADR 0028: 旧形式の部分名から修飾子と区切りの記号を除き、決められない部分名を移行しない](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md)）。

手元の adr-guard はすでに v0.13.1 に更新されている。
手元のpre-commit hookは、新しいADRを旧形式で書くと拒否し、コミット済みのADRのステータスの行を旧形式のまま書き換えても拒否する（「調査した事項」）。
そのため、ADRを足すたびに、またステータスの行を変えるたびに、新形式と旧形式が混ざっていく。
CIと手元の版をそろえ、すべてのADRのステータスの行を一度に新形式に移す。

## 調査した事項

次のことを、手元で実行して確かめた。

- `adr-guard -V` は `adr-guard 0.13.1` を表示した。
- 移行スクリプトは、adr-guard の v0.13.1 のソースの `scripts/migrate-adr-status` を使った。このスクリプトは、`cargo run --example migrate_adr_status` で example の `migrate_adr_status` をビルドして実行する。以下では、ビルドした `migrate_adr_status` を直接実行した。
- このリポジトリのルート（origin/main の a9d23b2）で `migrate_adr_status --dry-run` を実行すると、終了コード0で終わり、標準エラーの最後の行は `移行できる: 20件、移行済み: 0件、移行できない: 0件` だった。作業ツリーは書きかわらなかった。
  - 本ADRの草稿を置いた作業ツリーでは、最後の行は `移行できる: 20件、移行済み: 1件、移行できない: 0件` だった。本ADRは新形式なので、移行済みに数えられる。
  - 差分の行は、v0.13.0（`0096636`）の移行スクリプトを同じコミットで実行したときと同じだった。このリポジトリの部分名は、tett23/adr-guard の [ADR 0028](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md) の修正の影響を受けない。
  - [ADR 0020](0020-update-adr-guard-to-v0.10.0.md) の行は `Status: accepted` に、[ADR 0019](0019-update-adr-guard-to-v0.8.3.md) の `ステータス: 採択→指定するバージョンは[ADR 0020](...)` は `Status: superseded [{"by":["docs/adr/0020-update-adr-guard-to-v0.10.0.md"],"part":"指定するバージョン"}]` に移る。20件のうち、`accepted` が7件、`superseded` が13件である。
- 同じコミットの複製で `migrate_adr_status --write` を実行すると、20件のADRのステータスの行だけが書きかわった（`20 files changed, 20 insertions(+), 20 deletions(-)`）。その後の `--dry-run` は `移行できる: 0件、移行済み: 20件、移行できない: 0件` を表示した。
  - 書き換えた後の複製で、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` は終了コード0で終わった。`adr-guard unimplemented` は何も表示しなかった。
  - 書き換えをステージして `adr-guard pre-commit --allow-status --allow-links` は終了コード0で終わった。コミットした後、`adr-guard check-range HEAD~1 HEAD --allow-status --allow-links` も終了コード0で終わった。
- 移行していない複製（別のリポジトリ shorten-path で試した）で、手元の adr-guard（v0.13.1）の pre-commit は次を拒否した。
  - `ステータス: 提案` の新しいADR。診断は `` 新形式のステータス行は `Status: ` で始めてください `` だった。
  - `Status: accepted` の新しいADR。診断は `許可されていない状態遷移: Absent -> accepted` だった。
  - コミット済みのADRの `ステータス: 採択` を、旧形式のまま `ステータス: 採択→指定するバージョンは…` に書き換えた変更。診断は `` 新形式のステータス行は `Status: ` で始めてください `` で、終了コードは1だった。
  - `Status: proposed` の新しいADRは許可した。
- v0.13.1 の `action.yml` の入力は `base`、`head`、`version`、`allow-status`、`allow-links`、`links`、`check-status`、`check-numbers`、`allow-renumber` である。`version` を指定しないと、action自身の `Cargo.toml` の版（`v0.13.1`）のアーカイブ `adr-guard-v0.13.1-x86_64-unknown-linux-gnu.tar.gz` を取得する。
- 実装の手順を、origin/main の複製で次のとおりに試した。どの段階でも pre-commit（`--allow-status --allow-links`）は終了コード0で終わった。
  1. 本ADRを `Status: proposed` のまま足してコミットした。
  2. 移行スクリプトの `--write` を実行した。最後の行は `移行できる: 20件、移行済み: 1件、移行できない: 0件` で、本ADRは移行済みに数えられた。
  3. 本ADRを `Status: accepted` に、[ADR 0020](0020-update-adr-guard-to-v0.10.0.md) を「実装すること」に書いた `superseded` の行にして、移行とあわせて一つのコミットにした。
  - その後、`adr-guard check-range <origin/main> HEAD --allow-status --allow-links`、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` は終了コード0で終わった。
  - 移行を一つのコミットにし、本ADRの採択と [ADR 0020](0020-update-adr-guard-to-v0.10.0.md) の変更を次のコミットに分けた場合も、同じ結果だった。このとき `adr-guard trace docs/adr/0020-update-adr-guard-to-v0.10.0.md` は、[ADR 0020](0020-update-adr-guard-to-v0.10.0.md) から本ADRへの、部分「指定するバージョン」の辺を表示した。

## 実装すること

- `.github/workflows/ci.yml` の `adr` ジョブのactionを `uses: tett23/adr-guard@v0.13.1` にする。入力 `allow-status: true` と `allow-links: true`、`base`、`head` はそのままとし、ほかの入力は指定しない。
  - actionの上のADRの番号を並べたコメントに 0021 を加える。
- このリポジトリのルートで、adr-guard の v0.13.1 の移行スクリプトを `--dry-run` で実行し、`移行できない: 0件` で終わることを確かめてから、`--write` で全ADRのステータスの行を新形式に書き換える。adr-guard のソースから実行するときは `scripts/migrate-adr-status --dry-run` と `scripts/migrate-adr-status --write` である。書き換えの結果は手で直さない。
- [ADR 0020](0020-update-adr-guard-to-v0.10.0.md) のステータスの行を `Status: superseded [{"by":["docs/adr/0021-update-adr-guard-to-v0.13.1.md"],"part":"指定するバージョン"}]` にする。部分の名前は、移行した後の [ADR 0011](0011-pin-adr-guard-action-by-version.md)、[0012](0012-update-adr-guard-to-v0.3.0.md)、[0014](0014-update-adr-guard-to-v0.5.0.md)、[0015](0015-update-adr-guard-to-v0.8.0.md)、[0017](0017-update-adr-guard-to-v0.8.1.md)、[0018](0018-update-adr-guard-to-v0.8.2.md)、[0019](0019-update-adr-guard-to-v0.8.3.md) のステータスの行と同じ「指定するバージョン」とする。
- `.githooks/pre-commit` のコマンドは `exec adr-guard pre-commit --allow-status --allow-links` のままとする。コメントの「ステータス:」を「Status:」にし、ADRの番号に 0021 を加える。
- `README.md` の開発の節を v0.13.1 に合わせる。
  - adr-guard のインストール手順のタグを v0.13.1 にする。
  - ステータスの行の説明を新形式（行頭が `Status:`）にし、新しいADRは `Status: proposed` でコミットすること、採択されたがまだ実装されていないADRは `Status: accepted-pending` とすること、修正されたADRは `superseded` とすることを書く。
  - pre-commit hookとCIが状態遷移を検査し、その検査は `ADR_GUARD_ALLOW` とトレーラーでは免除されないことを書く。
- `docs/specifications.md` のADRの記述とCIの記述を v0.13.1 に合わせる。
  - ステータスの行の形、新しいADRを `Status: proposed` でコミットすること、状態遷移の検査、未実装のADRのステータスを `accepted-pending` とすることを書き、本ADRへリンクする。
  - `adr` ジョブのactionのバージョンを `v0.13.1` にし、ADRの並びに本ADRを加える。

## 実装しないこと

- actionの指定方法（バージョンのタグ）の変更。[ADR 0011](0011-pin-adr-guard-action-by-version.md) のままとする。
- Claude Codeのhookの変更。`.claude/settings.json` は `adr-guard claude-hook --allow-status` のままとする（[ADR 0016](0016-allow-status-in-claude-hook.md)）。
- Codex のhook（`adr-guard codex-hook`）の導入。
- 移行スクリプトの結果を超えた、ステータスの行の書き直し（`kind` や `note` を足すことなど）。
- ADRの本文と、ステータスの行以外の行の変更。
- 手元の adr-guard の更新の自動化。

## 不変条件

なし。設定の値と移行の手順を決めるADRで、入力によって変わる振る舞いを足さない。

## テスト設計

- 手元の adr-guard（v0.13.1）で、次を確かめる。
  - 書き換える前に移行スクリプトの `--dry-run` が終了コード0で終わり、`移行できない: 0件` を表示する。本ADRは新形式なので「移行済み」に数えられる。書き換えた後の `--dry-run` は `移行できる: 0件` を表示する。
  - `adr-guard links --fix` の後に、`adr-guard links`、`adr-guard check-status`、`adr-guard check-numbers` が終了コード0で終わる。
  - `adr-guard unimplemented` の出力が、書き換える前と同じである（いまは空）。
  - 各コミットでpre-commit hookが許可する。実装のコミットには、全ADRの移行と、[ADR 0020](0020-update-adr-guard-to-v0.10.0.md) の `superseded` への変更と、本ADRの `proposed` から `accepted` への変更が含まれる。
  - `adr-guard check-range origin/main HEAD --allow-status --allow-links` が終了コード0で終わる。
  - `adr-guard trace docs/adr/0020-update-adr-guard-to-v0.10.0.md` が、本ADRへの部分「指定するバージョン」の辺を表示する。
- `.github/workflows/ci.yml` がYAMLとして読めることを確かめる。
- 変更のプルリクエストのCIで、`adr` ジョブがv0.13.1のactionで実行され、成功することを確かめる。
  - ジョブのログで、`adr-guard-v0.13.1-x86_64-unknown-linux-gnu.tar.gz` のチェックサムが検証され、`check-range --allow-status --allow-links`、`links`、`check-status`、`check-numbers` が実行されていることを確かめる。

## トレードオフ

- ADRのステータスの行が英語のトークンと JSON になり、日本語の形より読みにくい。代わりに、状態遷移の検査と `adr-guard trace` が使え、旧形式と新形式が混ざらない。
- 一度に20件のADRのステータスの行を書き換えるので、差分が大きい。書き換えは移行スクリプトの結果だけで、手で直さないので、`--dry-run` の差分と比べて確かめられる。
- 状態遷移の検査は `ADR_GUARD_ALLOW` とトレーラーで免除されないので、誤ったステータスを緊急に戻す手段が減る。
- 手元の adr-guard が v0.12.0 より古いと、新形式の行と状態遷移が手元で検査されず、CIで初めて失敗する。READMEのインストール手順のタグを上げて補う。

## 参考文献

- tett23/adr-guard [ADR 0021: Codex の PreToolUse hook で、コミット済みのADRと記録の編集を拒否する](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0021-codex-hook.md)
- tett23/adr-guard [ADR 0022: 状態と遷移を別々の代数的データ型で表す](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0022-status-transitions.md)
- tett23/adr-guard [ADR 0023: trace で元の決定から修正ADRをたどる](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0023-trace-amendments.md)
- tett23/adr-guard [ADR 0024: 移行スクリプトに --dry-run を設け、失敗を最後まで数える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0024-migration-dry-run-and-error-count.md)
- tett23/adr-guard [ADR 0025: 新形式のステータス行に、実装待ちの修正、補足、注記、別のリポジトリの修正先を加える](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0025-extend-canonical-status-amendments.md)
- tett23/adr-guard [ADR 0026: 旧形式のステータス行の修正先を、リンク先と裸の番号からも読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0026-read-more-legacy-references.md)
- tett23/adr-guard [ADR 0027: 旧形式の `置換（…）` を、修正を受けた状態として読む](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0027-read-legacy-replaced-status.md)
- tett23/adr-guard [ADR 0028: 旧形式の部分名から修飾子と区切りの記号を除き、決められない部分名を移行しない](https://github.com/tett23/adr-guard/blob/v0.13.1/docs/adr/0028-fix-legacy-part-names.md)
- tett23/adr-guard [リリース v0.11.0](https://github.com/tett23/adr-guard/releases/tag/v0.11.0)、[リリース v0.12.0](https://github.com/tett23/adr-guard/releases/tag/v0.12.0)、[リリース v0.13.0](https://github.com/tett23/adr-guard/releases/tag/v0.13.0)、[リリース v0.13.1](https://github.com/tett23/adr-guard/releases/tag/v0.13.1)
