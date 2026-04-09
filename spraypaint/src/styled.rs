//! `Styled<T>` — wraps a value with a [`Style`], emitting ANSI on display.

use std::fmt;
use std::io::Write as _;

use crate::ansi;
use crate::style::Style;

/// A `Display` value paired with a [`Style`]. ANSI codes are emitted lazily
/// in the [`fmt::Display`] impl — no allocation until formatted.
#[derive(Debug, Clone, Copy)]
pub struct Styled<T> {
    pub(crate) inner: T,
    pub(crate) style: Style,
}

impl<T: fmt::Display> Styled<T> {
    pub(crate) fn new(inner: T, style: Style) -> Self {
        Self { inner, style }
    }

    /// Print to stdout with a trailing newline.
    pub fn paint(&self) {
        println!("{self}");
    }

    /// Print to stdout without a trailing newline (flushes immediately).
    pub fn paint_inline(&self) {
        print!("{self}");
        let _ = std::io::stdout().flush();
    }

    /// Print to stderr with a trailing newline.
    pub fn paint_err(&self) {
        eprintln!("{self}");
    }
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ansi::write_open(f, &self.style)?;
        self.inner.fmt(f)?;
        ansi::write_close(f, &self.style)
    }
}

impl<T: fmt::Display> Styled<T> {
    /// Layer an additional [`Style`] on top of the current one.
    pub fn and(mut self, extra: Style) -> Self {
        self.style = self.style.merge(extra);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::{set_color_level, ColorLevel};

    fn with_truecolor<F: FnOnce()>(f: F) {
        set_color_level(ColorLevel::TrueColor);
        f();
        set_color_level(ColorLevel::TrueColor);
    }

    #[test]
    fn display_contains_reset() {
        with_truecolor(|| {
            let s = Style::new().red().bold().apply("hello");
            let rendered = s.to_string();
            assert!(rendered.contains("\x1b["), "should contain escape");
            assert!(rendered.contains("\x1b[0m"), "should contain reset");
            assert!(rendered.contains("hello"), "should contain text");
        });
    }

    #[test]
    fn no_color_strips_ansi() {
        set_color_level(ColorLevel::None);
        let s = Style::new().red().bold().apply("hello");
        let rendered = s.to_string();
        assert_eq!(rendered, "hello");
        set_color_level(ColorLevel::TrueColor);
    }
}
