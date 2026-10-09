//! 設定の読み込みと検証
//!
//! .env（--env-file で指定したファイル、またはカレントディレクトリの .env）があればそこから、無ければ環境変数から読む。

use std::collections::HashMap;

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

/// 読んだ .env のパスと中身
pub struct Dotenv {
    pub path: String,
    pub text: String,
}

const REQUIRED_KEYS: [&str; 6] = [
    "EMAIL",
    "SEND_TO_KINDLE_EMAIL",
    "SMTP_HOST",
    "SMTP_PORT",
    "SMTP_USER_NAME",
    "SMTP_PASSWORD",
];

/// * `dotenv` - 読んだ .env。.env が無ければ None（環境変数を使う）
/// * `get_env` - 環境変数を読む関数
pub fn load_config(
    dotenv: Option<&Dotenv>,
    get_env: impl Fn(&str) -> Option<String>,
) -> Result<Config, String> {
    let (source, values): (String, HashMap<&str, String>) = match dotenv {
        Some(dotenv) => {
            let mut parsed = dotenv::parse(&dotenv.text);
            let values = REQUIRED_KEYS
                .map(|key| (key, parsed.remove(key).unwrap_or_default()))
                .into();
            // エラーメッセージではパスの後に空白を入れる
            (format!("{} ", dotenv.path), values)
        }
        None => (
            "環境変数".to_owned(),
            REQUIRED_KEYS
                .map(|key| (key, get_env(key).unwrap_or_default()))
                .into(),
        ),
    };
    validate(values, &source)
}

fn validate(mut values: HashMap<&str, String>, source: &str) -> Result<Config, String> {
    let missing: Vec<&str> = REQUIRED_KEYS
        .into_iter()
        .filter(|key| values[key].is_empty())
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "{source}に次の設定がありません: {}",
            missing.join(", ")
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

    fn dotenv(path: &str, text: &str) -> Dotenv {
        Dotenv {
            path: path.to_owned(),
            text: text.to_owned(),
        }
    }

    #[test]
    fn uses_dotenv_and_ignores_environment() {
        assert_eq!(
            load_config(Some(&dotenv(".env", TEST_DOTENV)), from_env(test_env())),
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
    fn uses_environment_without_dotenv() {
        assert_eq!(
            load_config(None, from_env(test_env())),
            Ok(Config {
                from: "env@example.com".into(),
                to: "env@kindle.com".into(),
                smtp: Smtp {
                    host: "smtp.env.example.com".into(),
                    port: 465,
                    user: "env-user".into(),
                    pass: "env-secret".into(),
                },
            }),
        );
    }

    #[test]
    fn lists_missing_keys_with_source() {
        let mut env = test_env();
        env.remove("SMTP_HOST");
        env.remove("SMTP_PASSWORD");
        assert_eq!(
            load_config(None, from_env(env)),
            Err("環境変数に次の設定がありません: SMTP_HOST, SMTP_PASSWORD".into()),
        );
    }

    #[test]
    fn empty_value_is_missing() {
        let text = TEST_DOTENV.replace("SMTP_USER_NAME=user", "SMTP_USER_NAME=");
        assert_eq!(
            load_config(Some(&dotenv(".env", &text)), from_env(HashMap::new())),
            Err(".env に次の設定がありません: SMTP_USER_NAME".into()),
        );
    }

    #[test]
    fn reports_specified_path() {
        let path = "/conf/kindle.env";
        assert!(load_config(Some(&dotenv(path, TEST_DOTENV)), from_env(HashMap::new())).is_ok());
        assert_eq!(
            load_config(Some(&dotenv(path, "EMAIL=me@example.com\n")), from_env(test_env())),
            Err("/conf/kindle.env に次の設定がありません: SEND_TO_KINDLE_EMAIL, SMTP_HOST, SMTP_PORT, SMTP_USER_NAME, SMTP_PASSWORD".into()),
        );
    }

    #[test]
    fn rejects_invalid_port() {
        for port in ["smtp", "0", "0587", "-1", "+587", "5 87"] {
            let mut env = test_env();
            env.insert("SMTP_PORT", port);
            assert_eq!(
                load_config(None, from_env(env)),
                Err(format!("SMTP_PORT は正の整数で指定してください: {port}")),
            );
        }
    }

    #[test]
    fn accepts_port_range_bounds() {
        for (value, port) in [("1", 1), ("65535", 65535)] {
            let mut env = test_env();
            env.insert("SMTP_PORT", value);
            assert_eq!(
                load_config(None, from_env(env)).map(|c| c.smtp.port),
                Ok(port)
            );
        }
    }

    #[test]
    fn rejects_port_above_range() {
        for port in ["65536", "99999999999999999999"] {
            let mut env = test_env();
            env.insert("SMTP_PORT", port);
            assert_eq!(
                load_config(None, from_env(env)),
                Err(format!("SMTP_PORT は 65535 以下で指定してください: {port}")),
            );
        }
    }
}
