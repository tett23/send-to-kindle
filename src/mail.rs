//! 送るメールの組み立て

use std::path::{Component, Path, PathBuf};

use crate::config::Config;

#[derive(Debug, PartialEq, Eq)]
pub struct Mail {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub text: String,
    pub attachment: Attachment,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Attachment {
    pub filename: String,
    pub path: PathBuf,
    pub content_type: &'static str,
}

/// * `file` - 送るファイル。相対パスなら `cwd` を基準に解決する
/// * `cwd` - カレントディレクトリ
pub fn build_mail(config: &Config, file: &str, cwd: &Path) -> Mail {
    let filename = Path::new(file)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    Mail {
        from: config.from.clone(),
        to: config.to.clone(),
        // Kindle のメールアドレスは件名が「変換」だと、送ったファイルを Kindle の形式に変換する
        subject: "変換".to_owned(),
        text: "book".to_owned(),
        attachment: Attachment {
            content_type: content_type(&filename),
            path: resolve(cwd, Path::new(file)),
            filename,
        },
    }
}

/// `cwd` を基準にパスを解決し、`.` と `..` を字句的に取り除く。
fn resolve(cwd: &Path, path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in cwd.join(path).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// Kindle が受けつける形式の拡張子から Content-Type を決める。
pub fn content_type(filename: &str) -> &'static str {
    let extension = Path::new(filename)
        .extension()
        .map(|ext| ext.to_string_lossy().to_ascii_lowercase());
    match extension.as_deref() {
        Some("epub") => "application/epub+zip",
        Some("pdf") => "application/pdf",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("rtf") => "application/rtf",
        Some("txt") => "text/plain",
        Some("htm" | "html") => "text/html",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("png") => "image/png",
        Some("bmp") => "image/bmp",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Smtp;

    fn test_config() -> Config {
        Config {
            from: "me@example.com".into(),
            to: "me@kindle.com".into(),
            smtp: Smtp {
                host: "smtp.example.com".into(),
                port: 587,
                user: "user".into(),
                pass: "secret".into(),
            },
        }
    }

    #[test]
    fn builds_mail_with_attachment() {
        assert_eq!(
            build_mail(&test_config(), "books/novel.epub", Path::new("/home/me")),
            Mail {
                from: "me@example.com".into(),
                to: "me@kindle.com".into(),
                subject: "変換".into(),
                text: "book".into(),
                attachment: Attachment {
                    filename: "novel.epub".into(),
                    path: "/home/me/books/novel.epub".into(),
                    content_type: "application/epub+zip",
                },
            },
        );
    }

    #[test]
    fn absolute_path_is_not_joined_with_cwd() {
        let mail = build_mail(&test_config(), "/tmp/novel.epub", Path::new("/home/me"));
        assert_eq!(mail.attachment.filename, "novel.epub");
        assert_eq!(mail.attachment.path, PathBuf::from("/tmp/novel.epub"));
    }

    #[test]
    fn resolves_dot_components() {
        let mail = build_mail(&test_config(), "../me/./a.pdf", Path::new("/home/me"));
        assert_eq!(mail.attachment.path, PathBuf::from("/home/me/a.pdf"));
    }

    #[test]
    fn content_type_by_extension() {
        assert_eq!(content_type("a.epub"), "application/epub+zip");
        assert_eq!(content_type("a.PDF"), "application/pdf");
        assert_eq!(content_type("a.Html"), "text/html");
        assert_eq!(content_type("a.jpeg"), "image/jpeg");
        assert_eq!(content_type("a.mobi"), "application/octet-stream");
        assert_eq!(content_type("README"), "application/octet-stream");
    }
}
