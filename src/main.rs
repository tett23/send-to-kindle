//! send-to-kindle [--env-file <パス>] <ファイル> - ファイルを Kindle のメールアドレスへ添付して送る
//! (docs/adr/0002)

mod args;
mod config;
mod dotenv;
mod help;
mod mail;

use std::io::ErrorKind;
use std::path::Path;
use std::process::ExitCode;

use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

use crate::config::{Config, Source};
use crate::mail::Mail;

/// エラーの表示と終了コードは main でまとめて扱う。
struct Failure(String);

impl<T: Into<String>> From<T> for Failure {
    fn from(message: T) -> Self {
        Failure(message.into())
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Failure(message)) => {
            eprintln!("send-to-kindle: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Failure> {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string()
                .map_err(|_| "引数を UTF-8 として読めません")
        })
        .collect::<Result<_, _>>()?;
    let args::CliArgs { file, env_file } = match args::parse_cli_args(&args)? {
        args::Command::Help => {
            print!("{}", help::help());
            return Ok(());
        }
        args::Command::Send(args) => args,
    };

    let config = config::load_config(&read_sources(env_file.as_deref())?)?;

    let cwd = std::env::current_dir()
        .map_err(|e| format!("カレントディレクトリを取得できません: {e}"))?;
    let mail = mail::build_mail(&config, &file, &cwd);
    let attachment = &mail.attachment;
    if !attachment.path.is_file() {
        return Err(format!("ファイルがありません: {}", attachment.path.display()).into());
    }
    let body = std::fs::read(&attachment.path)
        .map_err(|e| format!("ファイルを読めません: {}: {e}", attachment.path.display()))?;

    send(&config, &mail, body).map_err(|e| format!("送信できませんでした: {e}"))?;

    println!("送信しました: {} → {}", attachment.filename, config.to);
    Ok(())
}

/// 設定を読む場所を、優先順位の高い順に読む（ADR 0008）。
/// --env-file で指定したファイルが無ければエラーにし、それ以外のファイルは無ければ読まない
fn read_sources(env_file: Option<&str>) -> Result<Vec<Source>, Failure> {
    let get_env = |key: &str| std::env::var(key).ok();
    let mut sources = Vec::new();
    if let Some(path) = env_file {
        let text = read_optional(Path::new(path))?
            .ok_or_else(|| format!("指定した .env がありません: {path}"))?;
        sources.push(Source::dotenv(path, &text));
    }
    sources.push(Source::env(get_env));
    if let Some(text) = read_optional(Path::new(".env"))? {
        sources.push(Source::dotenv(".env", &text));
    }
    if let Some(path) = config::user_config_path(get_env)
        && let Some(text) = read_optional(&path)?
    {
        sources.push(Source::dotenv(&path.display().to_string(), &text));
    }
    Ok(sources)
}

/// ファイルを読む。無ければ None。
fn read_optional(path: &Path) -> Result<Option<String>, Failure> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("{} を読めません: {e}", path.display()).into()),
    }
}

fn send(config: &Config, mail: &Mail, body: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let attachment = &mail.attachment;
    let message = Message::builder()
        .from(mail.from.parse()?)
        .to(mail.to.parse()?)
        .subject(&mail.subject)
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::plain(mail.text.clone()))
                .singlepart(
                    Attachment::new(attachment.filename.clone())
                        .body(body, ContentType::parse(attachment.content_type)?),
                ),
        )?;

    let smtp = &config.smtp;
    let tls_parameters = TlsParameters::new(smtp.host.clone())?;
    // nodemailer の既定に合わせ、465 は最初から TLS、それ以外は対応していれば STARTTLS にする
    let tls = if smtp.port == 465 {
        Tls::Wrapper(tls_parameters)
    } else {
        Tls::Opportunistic(tls_parameters)
    };
    let transport = SmtpTransport::builder_dangerous(&smtp.host)
        .port(smtp.port)
        .tls(tls)
        .credentials(Credentials::new(smtp.user.clone(), smtp.pass.clone()))
        .build();
    transport.send(&message)?;
    Ok(())
}
