//! 引数の解釈

pub const USAGE: &str = "使い方: send-to-kindle [--env-file <パス>] <ファイル>";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Send(CliArgs),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CliArgs {
    pub file: String,
    pub env_file: Option<String>,
}

pub fn parse_cli_args(args: &[String]) -> Result<Command, String> {
    // `--` より前に -h / --help があれば、ほかの引数にかかわらずヘルプを表示する
    if args
        .iter()
        .take_while(|arg| *arg != "--")
        .any(|arg| arg == "-h" || arg == "--help")
    {
        return Ok(Command::Help);
    }

    let with_usage = |message: String| format!("{message}\n{USAGE}");

    let mut positionals = Vec::new();
    let mut env_file = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--" {
            positionals.extend(iter.by_ref().cloned());
        } else if arg == "--env-file" || arg == "-e" {
            match iter.next() {
                Some(value) => env_file = Some(value.clone()),
                None => return Err(with_usage(format!("{arg} には値が必要です"))),
            }
        } else if let Some(value) = arg.strip_prefix("--env-file=") {
            env_file = Some(value.to_owned());
        } else if arg.starts_with('-') && arg != "-" {
            return Err(with_usage(format!("知らないオプションです: {arg}")));
        } else {
            positionals.push(arg.clone());
        }
    }

    match <[String; 1]>::try_from(positionals) {
        Ok([file]) => Ok(Command::Send(CliArgs { file, env_file })),
        Err(_) => Err(USAGE.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Command, String> {
        parse_cli_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
    }

    fn ok(file: &str, env_file: Option<&str>) -> Result<Command, String> {
        Ok(Command::Send(CliArgs {
            file: file.to_owned(),
            env_file: env_file.map(str::to_owned),
        }))
    }

    #[test]
    fn file_only() {
        assert_eq!(parse(&["book.epub"]), ok("book.epub", None));
    }

    #[test]
    fn env_file_option() {
        assert_eq!(
            parse(&["--env-file", "/conf/kindle.env", "book.epub"]),
            ok("book.epub", Some("/conf/kindle.env")),
        );
    }

    #[test]
    fn env_file_option_with_equals_after_file() {
        assert_eq!(
            parse(&["book.epub", "--env-file=/conf/kindle.env"]),
            ok("book.epub", Some("/conf/kindle.env")),
        );
    }

    #[test]
    fn short_option() {
        assert_eq!(
            parse(&["-e", "kindle.env", "book.epub"]),
            ok("book.epub", Some("kindle.env"))
        );
    }

    #[test]
    fn missing_file_is_error() {
        assert_eq!(parse(&["--env-file", "kindle.env"]), Err(USAGE.to_owned()));
    }

    #[test]
    fn two_files_is_error() {
        assert_eq!(parse(&["a.epub", "b.epub"]), Err(USAGE.to_owned()));
    }

    #[test]
    fn missing_env_file_value_is_error() {
        let error = parse(&["book.epub", "--env-file"]).unwrap_err();
        assert!(error.ends_with(USAGE), "{error}");
    }

    #[test]
    fn unknown_option_is_error() {
        let error = parse(&["--unknown", "book.epub"]).unwrap_err();
        assert!(
            error.contains("--unknown") && error.ends_with(USAGE),
            "{error}"
        );
    }

    #[test]
    fn arguments_after_double_dash_are_files() {
        assert_eq!(parse(&["--", "-book.epub"]), ok("-book.epub", None));
    }

    #[test]
    fn single_dash_is_file() {
        assert_eq!(parse(&["-"]), ok("-", None));
    }

    #[test]
    fn help_options() {
        assert_eq!(parse(&["-h"]), Ok(Command::Help));
        assert_eq!(parse(&["--help"]), Ok(Command::Help));
    }

    #[test]
    fn help_wins_over_other_arguments() {
        for args in [
            &["--help", "book.epub", "-e", "kindle.env"][..],
            &["book.epub", "--env-file=kindle.env", "-h"],
            &["--unknown", "--help"],
            &["a.epub", "b.epub", "-h"],
        ] {
            assert_eq!(parse(args), Ok(Command::Help), "{args:?}");
        }
    }

    #[test]
    fn help_after_double_dash_is_file() {
        assert_eq!(parse(&["--", "--help"]), ok("--help", None));
    }
}
