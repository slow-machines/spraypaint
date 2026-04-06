//! Utility functions for working with ANSI-styled strings.

/// Remove all ANSI escape sequences from a string, returning plain text.
///
/// This is useful when you need to compute the visible width of a styled string,
/// write output to a file, or pass text to a library that does not expect escape codes.
///
/// Only CSI sequences (`ESC [` ...) are stripped; other ESC sequences (e.g. OSC
/// hyperlinks) are passed through unchanged with the ESC character dropped.
///
/// # Example
/// ```
/// use spraypaint::{Colorize, strip_ansi};
///
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

        // ESC character: check for CSI (`ESC [`)
        if chars.peek() == Some(&'[') {
            chars.next(); // consume '['
                          // Consume the CSI sequence until a "final byte" (0x40–0x7E = '@' to '~').
            for next in chars.by_ref() {
                if ('\x40'..='\x7e').contains(&next) {
                    break; // final byte consumed; sequence ends
                }
            }
        }
        // For any other ESC sequence, just drop the ESC and let the
        // following character be processed normally on the next iteration.
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_simple_color() {
        // ESC[31m = red; ESC[0m = reset
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
