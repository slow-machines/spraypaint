//! `Styled<T>` -- a zero-copy wrapper that applies a `Style` when displayed.

use std::fmt;
use std::io::Write as _;

use crate::ansi;
use crate::style::Style;

/// A value `T` with an attached [`Style`].
///
/// `Styled<T>` implements [`fmt::Display`], emitting ANSI escape codes around
/// the inner value's display output. No allocation occurs until the value is
/// formatted.
///
/// Obtained via the [`Colorize`](crate::Colorize) extension trait or
/// [`Style::apply`].
#[derive(Debug, Clone, Copy)]
pub struct Styled<T> {
    pub(crate) inner: T,
    pub(crate) style: Style,
}

impl<T: fmt::Display> Styled<T> {
    pub(crate) fn new(inner: T, style: Style) -> Self {
        Self { inner, style }
    }

    /// Print this styled value to stdout followed by a newline.
    ///
    /// Equivalent to `println!("{}", self)` but without the format string overhead.
    pub fn paint(&self) {
        println!("{self}");
    }

    /// Print this styled value to stdout **without** a trailing newline.
    pub fn paint_inline(&self) {
        print!("{self}");
        // Flush so partial lines appear immediately.
        let _ = std::io::stdout().flush();
    }

    /// Print this styled value to stderr followed by a newline.
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

/// Allow `Styled<T>` to itself be styled further (composing styles).
impl<T: fmt::Display> Styled<T> {
    /// Merge an additional `Style` on top of the current one.
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
        // Reset to avoid contaminating other tests
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
