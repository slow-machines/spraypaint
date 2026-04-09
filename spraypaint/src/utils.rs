//! ANSI utility helpers.

/// Strip CSI escape sequences (`ESC [ ... final_byte`), returning plain text.
///
/// ```
/// use spraypaint::{Colorize, strip_ansi};
/// let styled = "hello".red().bold().to_string();
/// assert_eq!(strip_ansi(&styled), "hello");
/// ```
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\x1b' {
            result.push(c);
            continue;
        }

        if chars.peek() == Some(&'[') {
            chars.next();
            // Consume until the CSI final byte (0x40–0x7E).
            for next in chars.by_ref() {
                if ('\x40'..='\x7e').contains(&next) {
                    break;
                }
            }
        }
        // Non-CSI ESC: drop the ESC, let the next char pass through.
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_simple_color() {
        assert_eq!(strip_ansi("\x1b[31mhello\x1b[0m"), "hello");
    }

    #[test]
    fn strips_truecolor() {
        assert_eq!(strip_ansi("\x1b[38;2;255;87;51mtest\x1b[0m"), "test");
    }

    #[test]
    fn strips_multiple_sequences() {
        assert_eq!(
            strip_ansi("\x1b[1m\x1b[4munderline bold\x1b[0m"),
            "underline bold"
        );
    }

    #[test]
    fn plain_text_unchanged() {
        assert_eq!(strip_ansi("no escapes here"), "no escapes here");
    }

    #[test]
    fn empty_string() {
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn unicode_preserved() {
        assert_eq!(strip_ansi("\x1b[32m日本語\x1b[0m"), "日本語");
    }
}
