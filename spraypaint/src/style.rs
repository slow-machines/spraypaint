//! The [`Style`] struct: a composable, copy-able bundle of colors and text attributes.

use bitflags::bitflags;

use crate::color::Color;

bitflags! {
    /// Text attribute flags used to build a [`Style`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Attrs: u16 {
        /// Bold / increased intensity (SGR 1).
        const BOLD          = 0b0000_0000_0001;
        /// Dim / faint / decreased intensity (SGR 2).
        const DIM           = 0b0000_0000_0010;
        /// Italic (SGR 3).
        const ITALIC        = 0b0000_0000_0100;
        /// Underline (SGR 4).
        const UNDERLINE     = 0b0000_0000_1000;
        /// Slow blink (SGR 5).
        const BLINK         = 0b0000_0001_0000;
        /// Rapid blink (SGR 6).
        const BLINK_FAST    = 0b0000_0010_0000;
        /// Reverse video / swap foreground and background (SGR 7).
        const REVERSE       = 0b0000_0100_0000;
        /// Conceal / hidden text (SGR 8).
        const HIDDEN        = 0b0000_1000_0000;
        /// Crossed-out / strikethrough (SGR 9).
        const STRIKETHROUGH = 0b0001_0000_0000;
    }
}

/// A composable style: optional foreground color, optional background color, and text attributes.
///
/// `Style` is `Copy`, so it is cheap to pass around and compose.
///
/// # Example
/// ```
/// use spraypaint::{Style, Color};
///
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
    /// Create a new, empty `Style` with no colors or attributes set.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            attrs: Attrs::empty(),
        }
    }

    /// Set the foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set the foreground from a 24-bit RGB triplet.
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Color::Rgb(r, g, b))
    }

    /// Set the foreground from a CSS hex string (`#RRGGBB` or `#RGB`).
    ///
    /// If the string is not a valid hex color, the style is returned unchanged.
    /// In debug builds, an assertion will fire to surface the mistake.
    pub fn hex(self, hex: &str) -> Self {
        if let Some(c) = Color::from_hex(hex) {
            self.fg(c)
        } else {
            debug_assert!(false, "invalid hex color: {hex}");
            self
        }
    }

    /// Set the background from a 24-bit RGB triplet.
    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Color::Rgb(r, g, b))
    }

    /// Set the background from a CSS hex string (`#RRGGBB` or `#RGB`).
    ///
    /// If the string is not a valid hex color, the style is returned unchanged.
    /// In debug builds, an assertion will fire to surface the mistake.
    pub fn on_hex(self, hex: &str) -> Self {
        if let Some(c) = Color::from_hex(hex) {
            self.bg(c)
        } else {
            debug_assert!(false, "invalid hex color: {hex}");
            self
        }
    }

    /// Merge `other` on top of `self`: `other`'s colors override `self`'s when set,
    /// and attributes are unioned.
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

    /// Apply this style to `val`, wrapping it in a [`Styled<T>`](crate::Styled).
    pub fn apply<T: std::fmt::Display>(self, val: T) -> crate::styled::Styled<T> {
        crate::styled::Styled::new(val, self)
    }

    // ── Attribute builder methods ─────────────────────────────────────────────

    /// Enable bold / increased intensity.
    pub fn bold(mut self) -> Self {
        self.attrs |= Attrs::BOLD;
        self
    }

    /// Enable dim / faint / decreased intensity.
    pub fn dim(mut self) -> Self {
        self.attrs |= Attrs::DIM;
        self
    }

    /// Enable italic text.
    pub fn italic(mut self) -> Self {
        self.attrs |= Attrs::ITALIC;
        self
    }

    /// Enable underline.
    pub fn underline(mut self) -> Self {
        self.attrs |= Attrs::UNDERLINE;
        self
    }

    /// Enable slow blink.
    pub fn blink(mut self) -> Self {
        self.attrs |= Attrs::BLINK;
        self
    }

    /// Enable rapid blink.
    pub fn blink_fast(mut self) -> Self {
        self.attrs |= Attrs::BLINK_FAST;
        self
    }

    /// Enable reverse video (swap foreground and background).
    pub fn reverse(mut self) -> Self {
        self.attrs |= Attrs::REVERSE;
        self
    }

    /// Enable hidden / concealed text.
    pub fn hidden(mut self) -> Self {
        self.attrs |= Attrs::HIDDEN;
        self
    }

    /// Enable strikethrough / crossed-out text.
    pub fn strikethrough(mut self) -> Self {
        self.attrs |= Attrs::STRIKETHROUGH;
        self
    }

    // ── Named foreground color shorthands ────────────────────────────────────

    /// Set foreground to black.
    pub fn black(self) -> Self {
        self.fg(Color::BLACK)
    }
    /// Set foreground to red.
    pub fn red(self) -> Self {
        self.fg(Color::RED)
    }
    /// Set foreground to green.
    pub fn green(self) -> Self {
        self.fg(Color::GREEN)
    }
    /// Set foreground to yellow.
    pub fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }
    /// Set foreground to blue.
    pub fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }
    /// Set foreground to magenta.
    pub fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }
    /// Set foreground to cyan.
    pub fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }
    /// Set foreground to white.
    pub fn white(self) -> Self {
        self.fg(Color::WHITE)
    }
    /// Set foreground to bright black (dark gray).
    pub fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }
    /// Set foreground to bright red.
    pub fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }
    /// Set foreground to bright green.
    pub fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }
    /// Set foreground to bright yellow.
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }
    /// Set foreground to bright blue.
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }
    /// Set foreground to bright magenta.
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }
    /// Set foreground to bright cyan.
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }
    /// Set foreground to bright white.
    pub fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    // ── Named background color shorthands ────────────────────────────────────

    /// Set background to black.
    pub fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }
    /// Set background to red.
    pub fn on_red(self) -> Self {
        self.bg(Color::RED)
    }
    /// Set background to green.
    pub fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }
    /// Set background to yellow.
    pub fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }
    /// Set background to blue.
    pub fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }
    /// Set background to magenta.
    pub fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }
    /// Set background to cyan.
    pub fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }
    /// Set background to white.
    pub fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }
    /// Set background to bright black (dark gray).
    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }
    /// Set background to bright red.
    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }
    /// Set background to bright green.
    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }
    /// Set background to bright yellow.
    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }
    /// Set background to bright blue.
    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }
    /// Set background to bright magenta.
    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }
    /// Set background to bright cyan.
    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }
    /// Set background to bright white.
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
