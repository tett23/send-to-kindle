use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::{Command, Output};
use std::thread::JoinHandle;

use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_send-to-kindle");

/// 環境変数を空にし、`cwd` で実行する。
fn run(cwd: &Path, args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .current_dir(cwd)
        .env_clear()
        .output()
        .unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_failure(output: &Output, message: &str) {
    assert_eq!(output.status.code(), Some(1));
    let stderr = stderr(output);
    assert!(stderr.starts_with("send-to-kindle: "), "{stderr}");
    assert!(stderr.contains(message), "{stderr}");
}

fn dotenv(port: u16) -> String {
    format!(
        "EMAIL=me@example.com
SEND_TO_KINDLE_EMAIL=me@kindle.com
SMTP_HOST=127.0.0.1
SMTP_PORT={port}
SMTP_USER_NAME=user
SMTP_PASSWORD=secret
"
    )
}

#[test]
fn fails_without_file_argument() {
    let dir = TempDir::new().unwrap();
    assert_failure(&run(dir.path(), &[]), "使い方: send-to-kindle");
}

#[test]
fn fails_with_unknown_option() {
    let dir = TempDir::new().unwrap();
    assert_failure(&run(dir.path(), &["--unknown", "book.epub"]), "--unknown");
}

#[test]
fn fails_when_specified_env_file_is_missing() {
    let dir = TempDir::new().unwrap();
    assert_failure(
        &run(dir.path(), &["--env-file", "missing.env", "book.epub"]),
        "指定した .env がありません: missing.env",
    );
}

#[test]
fn fails_when_config_is_missing() {
    let dir = TempDir::new().unwrap();
    assert_failure(
        &run(dir.path(), &["book.epub"]),
        "環境変数に次の設定がありません: EMAIL",
    );
}

#[test]
fn fails_when_file_is_missing() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join(".env"), dotenv(25)).unwrap();
    let output = run(dir.path(), &["book.epub"]);
    assert_failure(&output, "ファイルがありません: ");
    assert!(stderr(&output).contains("book.epub"));
}

#[test]
fn fails_when_send_fails() {
    // 何も待ち受けていないポートへ送る
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join(".env"), dotenv(port)).unwrap();
    std::fs::write(dir.path().join("book.epub"), "x").unwrap();
    assert_failure(&run(dir.path(), &["book.epub"]), "送信できませんでした: ");
}

/// 1 通だけ受けつける SMTP サーバー（TLS なし）。受け取ったコマンドとデータを返す。
fn smtp_server() -> (u16, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let handle = std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = stream;
        let mut transcript = String::new();
        let mut in_data = false;
        writer.write_all(b"220 localhost ESMTP\r\n").unwrap();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            transcript.push_str(&line);
            if in_data {
                if line == ".\r\n" {
                    in_data = false;
                    writer.write_all(b"250 OK\r\n").unwrap();
                }
                continue;
            }
            let command = line.to_ascii_uppercase();
            let reply: &[u8] = if command.starts_with("EHLO") {
                b"250-localhost\r\n250 AUTH PLAIN LOGIN\r\n"
            } else if command.starts_with("AUTH") {
                b"235 OK\r\n"
            } else if command.starts_with("DATA") {
                in_data = true;
                b"354 Go ahead\r\n"
            } else if command.starts_with("QUIT") {
                writer.write_all(b"221 Bye\r\n").unwrap();
                break;
            } else {
                b"250 OK\r\n"
            };
            writer.write_all(reply).unwrap();
        }
        transcript
    });
    (port, handle)
}

#[test]
fn sends_file_to_kindle() {
    let (port, server) = smtp_server();
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("kindle.env"), dotenv(port)).unwrap();
    std::fs::create_dir(dir.path().join("books")).unwrap();
    std::fs::write(dir.path().join("books/小説.epub"), "hello").unwrap();

    let output = run(dir.path(), &["books/小説.epub", "-e", "kindle.env"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "送信しました: 小説.epub → me@kindle.com\n"
    );

    let transcript = server.join().unwrap();
    let expected = [
        // AUTH PLAIN の資格情報（"\0user\0secret" の base64）
        "AUTH PLAIN AHVzZXIAc2VjcmV0",
        "MAIL FROM:<me@example.com>",
        "RCPT TO:<me@kindle.com>",
        "From: me@example.com",
        "To: me@kindle.com",
        // 「変換」の UTF-8 の base64
        "Subject: =?utf-8?b?5aSJ5o+b?=",
        "\r\nbook\r\n",
        "Content-Type: application/epub+zip",
        // 「小説.epub」を RFC 2231 でエンコードしたファイル名
        "filename*0*=utf-8''%E5%B0%8F%E8%AA%AC.epub",
        // 添付の中身
        "\r\nhello\r\n",
    ];
    for needle in expected {
        assert!(
            transcript.contains(needle),
            "{needle} が無い:\n{transcript}"
        );
    }
}

#[test]
fn fails_when_port_is_out_of_range() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join(".env"),
        dotenv(25).replace("=25\n", "=65536\n"),
    )
    .unwrap();
    std::fs::write(dir.path().join("book.epub"), "x").unwrap();
    assert_failure(
        &run(dir.path(), &["book.epub"]),
        "SMTP_PORT は 65535 以下で指定してください: 65536",
    );
}

#[test]
fn shows_help() {
    // 設定もファイルも無いディレクトリで実行する
    let dir = TempDir::new().unwrap();
    for option in ["--help", "-h"] {
        let output = run(dir.path(), &[option]);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("使い方: send-to-kindle"), "{stdout}");
    }
}
