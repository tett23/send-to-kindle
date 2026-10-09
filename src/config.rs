//! 設定の読み込みと検証
//!
//! 設定項目ごとに、優先順位の高い場所から順に探し、最初に見つかった値を使う（ADR 0008）。
//! 場所の優先順位は、--env-file で指定したファイル、環境変数、カレントディレクトリの .env、
//! ユーザーの設定ファイルの順。

use std::collections::HashMap;
use std::path::PathBuf;

use crate::dotenv;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub from: String,
    pub to: String,
    pub smtp: Smtp,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
}

/// 設定を読んだ場所と、そこで読んだ値
pub struct Source {
    /// エラーメッセージに出す場所の名前（ファイルのパス、または「環境変数」）
    pub label: String,
    pub values: HashMap<String, String>,
}

impl Source {
    pub fn dotenv(path: &str, text: &str) -> Source {
        Source {
            label: path.to_owned(),
            values: dotenv::parse(text),
        }
    }

    /// * `get_env` - 環境変数を読む関数
    pub fn env(get_env: impl Fn(&str) -> Option<String>) -> Source {
        Source {
            label: "環境変数".to_owned(),
            values: REQUIRED_KEYS
                .into_iter()
                .filter_map(|key| get_env(key).map(|value| (key.to_owned(), value)))
                .collect(),
        }
    }
}

pub const REQUIRED_KEYS: [&str; 6] = [
    "EMAIL",
    "SEND_TO_KINDLE_EMAIL",
    "SMTP_HOST",
    "SMTP_PORT",
    "SMTP_USER_NAME",
    "SMTP_PASSWORD",
];

/// ユーザーの設定ファイルのパス。`XDG_CONFIG_HOME` が絶対パスならその下、
/// そうでなければ `$HOME/.config` の下。どちらも使えなければ None。
///
/// * `get_env` - 環境変数を読む関数
pub fn user_config_path(get_env: impl Fn(&str) -> Option<String>) -> Option<PathBuf> {
    let config_home = get_env("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        // 空や相対パスは設定されていないものとして扱う（XDG Base Directory仕様）
        .filter(|path| path.is_absolute())
        .or_else(|| {
            get_env("HOME")
                .filter(|home| !home.is_empty())
                .map(|home| PathBuf::from(home).join(".config"))
        })?;
    Some(config_home.join("send-to-kindle").join(".env"))
}

/// * `sources` - 設定を読んだ場所。優先順位の高い順に並べる
pub fn load_config(sources: &[Source]) -> Result<Config, String> {
    let mut values: HashMap<&str, String> = HashMap::new();
    let mut missing = Vec::new();
    for key in REQUIRED_KEYS {
        // 空文字の値は、その場所に無いものとして次の場所を探す
        match sources
            .iter()
            .find_map(|source| source.values.get(key).filter(|value| !value.is_empty()))
        {
            Some(value) => {
                values.insert(key, value.clone());
            }
            None => missing.push(key),
        }
    }
    if !missing.is_empty() {
        let labels: Vec<&str> = sources.iter().map(|source| source.label.as_str()).collect();
        return Err(format!(
            "次の設定がありません: {}（読んだ場所: {}）",
            missing.join(", "),
            labels.join(", ")
        ));
    }

    let port = &values["SMTP_PORT"];
    if !is_positive_integer(port) {
        return Err(format!("SMTP_PORT は正の整数で指定してください: {port}"));
    }
    // 正の整数で u16 に収まらないものは、65535 より大きい
    let Ok(port) = port.parse::<u16>() else {
        return Err(format!("SMTP_PORT は 65535 以下で指定してください: {port}"));
    };

    let mut take = |key| values.remove(key).unwrap_or_default();
    Ok(Config {
        from: take("EMAIL"),
        to: take("SEND_TO_KINDLE_EMAIL"),
        smtp: Smtp {
            host: take("SMTP_HOST"),
            port,
            user: take("SMTP_USER_NAME"),
            pass: take("SMTP_PASSWORD"),
        },
    })
}

fn is_positive_integer(value: &str) -> bool {
    value.starts_with(|c: char| ('1'..='9').contains(&c))
        && value.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DOTENV: &str = "EMAIL=me@example.com
SEND_TO_KINDLE_EMAIL=me@kindle.com
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER_NAME=user
SMTP_PASSWORD=secret
";

    fn test_env() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("EMAIL", "env@example.com"),
            ("SEND_TO_KINDLE_EMAIL", "env@kindle.com"),
            ("SMTP_HOST", "smtp.env.example.com"),
            ("SMTP_PORT", "465"),
            ("SMTP_USER_NAME", "env-user"),
            ("SMTP_PASSWORD", "env-secret"),
        ])
    }

    fn from_env(env: HashMap<&'static str, &'static str>) -> impl Fn(&str) -> Option<String> {
        move |key| env.get(key).map(|v| v.to_string())
    }

    fn env_source(env: HashMap<&'static str, &'static str>) -> Source {
        Source::env(from_env(env))
    }

    fn port_config(port: &'static str) -> Result<Config, String> {
        let mut env = test_env();
        env.insert("SMTP_PORT", port);
        load_config(&[env_source(env)])
    }

    #[test]
    fn reads_all_keys_from_one_source() {
        assert_eq!(
            load_config(&[Source::dotenv(".env", TEST_DOTENV)]),
            Ok(Config {
                from: "me@example.com".into(),
                to: "me@kindle.com".into(),
                smtp: Smtp {
                    host: "smtp.example.com".into(),
                    port: 587,
                    user: "user".into(),
                    pass: "secret".into(),
                },
            }),
        );
    }

    #[test]
    fn higher_priority_source_wins_per_key() {
        let sources = [
            Source::dotenv("kindle.env", "SMTP_PASSWORD=from-env-file\n"),
            env_source(HashMap::from([
                ("SMTP_PASSWORD", "from-env"),
                ("SMTP_USER_NAME", "env-user"),
            ])),
            Source::dotenv(
                ".env",
                "SMTP_USER_NAME=dotenv-user\nSEND_TO_KINDLE_EMAIL=cwd@kindle.com\n",
            ),
            Source::dotenv("/home/me/.config/send-to-kindle/.env", TEST_DOTENV),
        ];
        assert_eq!(
            load_config(&sources),
            Ok(Config {
                from: "me@example.com".into(),
                to: "cwd@kindle.com".into(),
                smtp: Smtp {
                    host: "smtp.example.com".into(),
                    port: 587,
                    user: "env-user".into(),
                    pass: "from-env-file".into(),
                },
            }),
        );
    }

    #[test]
    fn empty_value_falls_through_to_next_source() {
        let sources = [
            Source::dotenv(".env", "SMTP_USER_NAME=\n"),
            Source::dotenv("user.env", TEST_DOTENV),
        ];
        assert_eq!(
            load_config(&sources).map(|c| c.smtp.user),
            Ok("user".into())
        );
    }

    #[test]
    fn lists_missing_keys_with_sources_in_priority_order() {
        let mut env = test_env();
        env.remove("SMTP_HOST");
        env.remove("SMTP_PASSWORD");
        let sources = [
            env_source(env),
            Source::dotenv(".env", "SMTP_PASSWORD=\n"),
            Source::dotenv(
                "/home/me/.config/send-to-kindle/.env",
                "EMAIL=x@example.com\n",
            ),
        ];
        assert_eq!(
            load_config(&sources),
            Err("次の設定がありません: SMTP_HOST, SMTP_PASSWORD（読んだ場所: 環境変数, .env, /home/me/.config/send-to-kindle/.env）".into()),
        );
    }

    #[test]
    fn rejects_invalid_port() {
        for port in ["smtp", "0", "0587", "-1", "+587", "5 87"] {
            assert_eq!(
                port_config(port),
                Err(format!("SMTP_PORT は正の整数で指定してください: {port}")),
            );
        }
    }

    #[test]
    fn validates_resolved_port() {
        // 優先される場所の不正な値が使われ、検証される
        let sources = [
            Source::dotenv("kindle.env", "SMTP_PORT=smtp\n"),
            Source::dotenv(".env", TEST_DOTENV),
        ];
        assert_eq!(
            load_config(&sources),
            Err("SMTP_PORT は正の整数で指定してください: smtp".into()),
        );
    }

    #[test]
    fn accepts_port_range_bounds() {
        for (value, port) in [("1", 1), ("65535", 65535)] {
            assert_eq!(port_config(value).map(|c| c.smtp.port), Ok(port));
        }
    }

    #[test]
    fn rejects_port_above_range() {
        for port in ["65536", "99999999999999999999"] {
            assert_eq!(
                port_config(port),
                Err(format!("SMTP_PORT は 65535 以下で指定してください: {port}")),
            );
        }
    }

    #[test]
    fn user_config_path_prefers_absolute_xdg_config_home() {
        let env = from_env(HashMap::from([
            ("XDG_CONFIG_HOME", "/xdg"),
            ("HOME", "/home/me"),
        ]));
        assert_eq!(
            user_config_path(env),
            Some(PathBuf::from("/xdg/send-to-kindle/.env"))
        );
    }

    #[test]
    fn user_config_path_falls_back_to_home() {
        for xdg in [None, Some(""), Some("relative/xdg")] {
            let mut env = HashMap::from([("HOME", "/home/me")]);
            if let Some(xdg) = xdg {
                env.insert("XDG_CONFIG_HOME", xdg);
            }
            assert_eq!(
                user_config_path(from_env(env)),
                Some(PathBuf::from("/home/me/.config/send-to-kindle/.env")),
                "{xdg:?}",
            );
        }
    }

    #[test]
    fn user_config_path_is_none_without_home() {
        assert_eq!(user_config_path(from_env(HashMap::new())), None);
        assert_eq!(
            user_config_path(from_env(HashMap::from([
                ("HOME", ""),
                ("XDG_CONFIG_HOME", "relative")
            ]))),
            None
        );
    }
}
