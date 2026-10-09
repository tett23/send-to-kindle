//! ヘルプの文面

use crate::args::USAGE;

pub fn help() -> String {
    format!(
        "send-to-kindle {version}
ファイルを Kindle のメールアドレスへ添付して送る。

{USAGE}

引数:
  <ファイル>  送るファイル。相対パスはカレントディレクトリを基準にする

オプション:
  -e, --env-file <パス>  設定を読むファイル。指定したファイルが無ければエラーにする
  -h, --help             このヘルプを表示する

設定:
  次の場所を、この優先順位で読む。項目ごとに、優先順位の高い場所から順に探し、
  最初に見つかった値を使う（空文字は無いものとして扱う）。場所ごとに一部の項目だけを書いてよい。
    1. --env-file で指定したファイル
    2. 環境変数
    3. カレントディレクトリの .env
    4. ユーザーの設定ファイル
       $XDG_CONFIG_HOME/send-to-kindle/.env（XDG_CONFIG_HOME が絶対パスのとき）
       それ以外は ~/.config/send-to-kindle/.env

  EMAIL                 送信元のメールアドレス（Kindle の承認済みアドレス）
  SEND_TO_KINDLE_EMAIL  Kindle のメールアドレス
  SMTP_HOST             SMTP サーバーのホスト
  SMTP_PORT             SMTP サーバーのポート（1〜65535）。465 は最初から TLS で、
                        それ以外はサーバーが対応していれば STARTTLS で接続する
  SMTP_USER_NAME        SMTP の認証に使うユーザー名
  SMTP_PASSWORD         SMTP の認証に使うパスワード

送信内容:
  件名「変換」、本文「book」でファイルを添付して送る。件名が「変換」なので、
  Kindle は送ったファイルを Kindle の形式に変換する。
  Content-Type を判定する拡張子: epub, pdf, doc, docx, rtf, txt, htm, html,
  jpg, jpeg, gif, png, bmp（それ以外は application/octet-stream）

例:
  send-to-kindle book.epub
  send-to-kindle --env-file ./kindle.env book.epub
  SMTP_PASSWORD=... send-to-kindle book.epub
",
        version = env!("CARGO_PKG_VERSION"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::REQUIRED_KEYS;

    #[test]
    fn contains_usage_and_version() {
        let help = help();
        assert!(help.contains(USAGE));
        assert!(help.starts_with(&format!("send-to-kindle {}\n", env!("CARGO_PKG_VERSION"))));
    }

    #[test]
    fn contains_user_config_path() {
        let help = help();
        assert!(help.contains("~/.config/send-to-kindle/.env"));
        assert!(help.contains("$XDG_CONFIG_HOME/send-to-kindle/.env"));
    }

    #[test]
    fn contains_all_required_keys() {
        let help = help();
        for key in REQUIRED_KEYS {
            assert!(help.contains(&format!("\n  {key} ")), "{key}");
        }
    }
}
