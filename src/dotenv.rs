//! `.env` の解析（ADR 0002 で定めた範囲だけに対応する）

use std::collections::HashMap;

pub fn parse(text: &str) -> HashMap<String, String> {
    text.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let line = line.strip_prefix("export ").map_or(line, str::trim_start);
    let (key, value) = line.split_once('=')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    Some((key.to_owned(), parse_value(value.trim())))
}

fn parse_value(raw: &str) -> String {
    if let Some(rest) = raw.strip_prefix('\'')
        && let Some((inner, _)) = rest.split_once('\'')
    {
        return inner.to_owned();
    }
    if let Some(rest) = raw.strip_prefix('"')
        && let Some(value) = parse_double_quoted(rest)
    {
        return value;
    }
    strip_comment(raw).to_owned()
}

/// 閉じるダブルクォートまでを、エスケープを解釈して読む。閉じていなければ None。
fn parse_double_quoted(rest: &str) -> Option<String> {
    let mut value = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(value),
            '\\' => match chars.next()? {
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                other => {
                    value.push('\\');
                    value.push(other);
                }
            },
            other => value.push(other),
        }
    }
    None
}

/// 空白に続く `#` 以降を取り除く。
fn strip_comment(raw: &str) -> &str {
    let end = raw
        .char_indices()
        .find(|&(i, c)| c == '#' && raw[..i].ends_with(char::is_whitespace))
        .map_or(raw.len(), |(i, _)| i);
    raw[..end].trim_end()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get(text: &str, key: &str) -> Option<String> {
        parse(text).remove(key)
    }

    #[test]
    fn ignores_blank_comment_and_invalid_lines() {
        let values = parse("\n# comment\n  # indented\nINVALID\n=no-key\nKEY=value\n");
        assert_eq!(
            values,
            HashMap::from([("KEY".to_owned(), "value".to_owned())])
        );
    }

    #[test]
    fn ignores_export_and_trims() {
        assert_eq!(
            get("export  KEY =  value  ", "KEY").as_deref(),
            Some("value")
        );
    }

    #[test]
    fn single_quoted_value_is_literal() {
        assert_eq!(
            get(r"KEY='a\n # b' # c", "KEY").as_deref(),
            Some(r"a\n # b")
        );
    }

    #[test]
    fn double_quoted_value_interprets_escapes() {
        assert_eq!(
            get(r#"KEY="a\nb\rc\td\"e\\f # g" # h"#, "KEY").as_deref(),
            Some("a\nb\rc\td\"e\\f # g"),
        );
    }

    #[test]
    fn unquoted_value_strips_comment_after_whitespace() {
        assert_eq!(get("KEY=a#b # c", "KEY").as_deref(), Some("a#b"));
    }

    #[test]
    fn unclosed_quote_is_treated_as_unquoted() {
        assert_eq!(get("KEY=\"abc # c", "KEY").as_deref(), Some("\"abc"));
    }

    #[test]
    fn does_not_expand_variables() {
        assert_eq!(get("A=1\nKEY=${A}", "KEY").as_deref(), Some("${A}"));
    }

    #[test]
    fn later_key_wins() {
        assert_eq!(get("KEY=1\nKEY=2", "KEY").as_deref(), Some("2"));
    }

    #[test]
    fn empty_value() {
        assert_eq!(get("KEY=", "KEY").as_deref(), Some(""));
    }

    #[test]
    fn crlf_line_endings() {
        assert_eq!(
            get("KEY=value\r\nOTHER=x\r\n", "KEY").as_deref(),
            Some("value")
        );
    }
}
