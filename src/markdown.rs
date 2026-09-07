//! MarkdownV2 escaping utilities shared between the Telegram command handler
//! (src/bin/main.rs) and balance formatting (src/balance/mod.rs). Kept in one
//! place so the escaped-character set can't drift out of sync between the two
//! call sites — previously main.rs had its own private copy that balance/mod.rs
//! could not reach, which is exactly how the missing-escaping bug happened.

/// Escapes special MarkdownV2 characters in dynamic data before inserting into a message.
pub fn escape_markdown_v2(text: &str) -> String {
    const SPECIAL: &[char] = &[
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=',
        '|', '{', '}', '.', '!',
    ];
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if SPECIAL.contains(&c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}
