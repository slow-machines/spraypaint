//! [`Style`]: composable foreground, background, and text attributes.

use bitflags::bitflags;

use crate::color::Color;

bitflags! {
    /// SGR text attribute flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Attrs: u16 {
        /// SGR 1
        const BOLD          = 0b0000_0000_0001;
        /// SGR 2
        const DIM           = 0b0000_0000_0010;
        /// SGR 3
        const ITALIC        = 0b0000_0000_0100;
        /// SGR 4
        const UNDERLINE     = 0b0000_0000_1000;
        /// SGR 5
        const BLINK         = 0b0000_0001_0000;
        /// SGR 6
        const BLINK_FAST    = 0b0000_0010_0000;
        /// SGR 7
        const REVERSE       = 0b0000_0100_0000;
        /// SGR 8
        const HIDDEN        = 0b0000_1000_0000;
        /// SGR 9
        const STRIKETHROUGH = 0b0001_0000_0000;
    }
}

/// Foreground + background + attributes bundle. `Copy` and composable.
///
/// ```
/// use spraypaint::{Style, Color};
/// let s = Style::new().fg(Color::RED).bold().underline();
/// ```
#[must_use = "Style is a builder; assign or use the result"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) attrs: Attrs,
}

impl Style {
    /// Empty style.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            attrs: Attrs::empty(),
        }
    }

    /// Foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Foreground from RGB.
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Color::Rgb(r, g, b))
    }

    /// Foreground from hex (`#RRGGBB` or `#RGB`). No-op on invalid input
    /// (debug-asserts in dev builds).
    pub fn hex(self, hex: &str) -> Self {
        if let Some(c) = Color::from_hex(hex) {
            self.fg(c)
        } else {
            debug_assert!(false, "invalid hex color: {hex}");
            self
        }
    }

    /// Background from RGB.
    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Color::Rgb(r, g, b))
    }

    /// Background from hex. Same no-op/assert behavior as [`hex`](Self::hex).
    pub fn on_hex(self, hex: &str) -> Self {
        if let Some(c) = Color::from_hex(hex) {
            self.bg(c)
        } else {
            debug_assert!(false, "invalid hex color: {hex}");
            self
        }
    }

    /// Overlay `other` onto `self` (colors override, attributes union).
    pub fn merge(mut self, other: Style) -> Self {
        if other.fg.is_some() {
            self.fg = other.fg;
        }
        if other.bg.is_some() {
            self.bg = other.bg;
        }
        self.attrs |= other.attrs;
        self
    }

    /// Wrap `val` in a [`Styled<T>`](crate::Styled) with this style.
    pub fn apply<T: std::fmt::Display>(self, val: T) -> crate::styled::Styled<T> {
        crate::styled::Styled::new(val, self)
    }

    // Attributes

    /// Bold.
    pub fn bold(mut self) -> Self {
        self.attrs |= Attrs::BOLD;
        self
    }
    /// Dim / faint.
    pub fn dim(mut self) -> Self {
        self.attrs |= Attrs::DIM;
        self
    }
    /// Italic.
    pub fn italic(mut self) -> Self {
        self.attrs |= Attrs::ITALIC;
        self
    }
    /// Underline.
    pub fn underline(mut self) -> Self {
        self.attrs |= Attrs::UNDERLINE;
        self
    }
    /// Slow blink.
    pub fn blink(mut self) -> Self {
        self.attrs |= Attrs::BLINK;
        self
    }
    /// Rapid blink.
    pub fn blink_fast(mut self) -> Self {
        self.attrs |= Attrs::BLINK_FAST;
        self
    }
    /// Reverse video.
    pub fn reverse(mut self) -> Self {
        self.attrs |= Attrs::REVERSE;
        self
    }
    /// Hidden / concealed.
    pub fn hidden(mut self) -> Self {
        self.attrs |= Attrs::HIDDEN;
        self
    }
    /// Strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.attrs |= Attrs::STRIKETHROUGH;
        self
    }

    // Named foreground shorthands

    /// Black foreground.
    pub fn black(self) -> Self {
        self.fg(Color::BLACK)
    }
    /// Red foreground.
    pub fn red(self) -> Self {
        self.fg(Color::RED)
    }
    /// Green foreground.
    pub fn green(self) -> Self {
        self.fg(Color::GREEN)
    }
    /// Yellow foreground.
    pub fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }
    /// Blue foreground.
    pub fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }
    /// Magenta foreground.
    pub fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }
    /// Cyan foreground.
    pub fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }
    /// White foreground.
    pub fn white(self) -> Self {
        self.fg(Color::WHITE)
    }
    /// Bright black (dark gray) foreground.
    pub fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }
    /// Bright red foreground.
    pub fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }
    /// Bright green foreground.
    pub fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }
    /// Bright yellow foreground.
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }
    /// Bright blue foreground.
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }
    /// Bright magenta foreground.
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }
    /// Bright cyan foreground.
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }
    /// Bright white foreground.
    pub fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    // Named background shorthands

    /// Black background.
    pub fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }
    /// Red background.
    pub fn on_red(self) -> Self {
        self.bg(Color::RED)
    }
    /// Green background.
    pub fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }
    /// Yellow background.
    pub fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }
    /// Blue background.
    pub fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }
    /// Magenta background.
    pub fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }
    /// Cyan background.
    pub fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }
    /// White background.
    pub fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }
    /// Bright black (dark gray) background.
    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }
    /// Bright red background.
    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }
    /// Bright green background.
    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }
    /// Bright yellow background.
    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }
    /// Bright blue background.
    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }
    /// Bright magenta background.
    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }
    /// Bright cyan background.
    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }
    /// Bright white background.
    pub fn on_bright_white(self) -> Self {
        self.bg(Color::BRIGHT_WHITE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_chaining() {
        let s = Style::new().red().bold().underline();
        assert_eq!(s.fg, Some(Color::RED));
        assert!(s.attrs.contains(Attrs::BOLD));
        assert!(s.attrs.contains(Attrs::UNDERLINE));
    }

    #[test]
    fn merge_styles() {
        let base = Style::new().red().bold();
        let overlay = Style::new().blue().italic();
        let merged = base.merge(overlay);
        assert_eq!(merged.fg, Some(Color::BLUE));
        assert!(merged.attrs.contains(Attrs::BOLD));
        assert!(merged.attrs.contains(Attrs::ITALIC));
    }
}
