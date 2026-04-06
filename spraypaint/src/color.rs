//! Color types supporting Basic 16, Xterm 256, and 24-bit RGB.

/// A named ANSI 4-bit color (the standard 16-color palette).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColor {
    /// Black (SGR 30 / 40).
    Black,
    /// Red (SGR 31 / 41).
    Red,
    /// Green (SGR 32 / 42).
    Green,
    /// Yellow (SGR 33 / 43).
    Yellow,
    /// Blue (SGR 34 / 44).
    Blue,
    /// Magenta (SGR 35 / 45).
    Magenta,
    /// Cyan (SGR 36 / 46).
    Cyan,
    /// White (SGR 37 / 47).
    White,
    /// Bright black / dark gray (SGR 90 / 100).
    BrightBlack,
    /// Bright red (SGR 91 / 101).
    BrightRed,
    /// Bright green (SGR 92 / 102).
    BrightGreen,
    /// Bright yellow (SGR 93 / 103).
    BrightYellow,
    /// Bright blue (SGR 94 / 104).
    BrightBlue,
    /// Bright magenta (SGR 95 / 105).
    BrightMagenta,
    /// Bright cyan (SGR 96 / 106).
    BrightCyan,
    /// Bright white (SGR 97 / 107).
    BrightWhite,
}

impl AnsiColor {
    /// Returns the foreground SGR code for this color.
    pub(crate) fn fg_code(self) -> u8 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
        }
    }

    /// Returns the background SGR code for this color.
    pub(crate) fn bg_code(self) -> u8 {
        self.fg_code() + 10
    }
}

/// A terminal color: basic 16-color, 256-color Xterm, or 24-bit RGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// One of the 16 standard ANSI colors.
    Ansi(AnsiColor),
    /// An Xterm 256-color palette index (0–255).
    Xterm(u8),
    /// A 24-bit RGB truecolor value.
    Rgb(u8, u8, u8),
}

impl Color {
    // ── Named constants ──────────────────────────────────────────────────────

    /// ANSI black.
    pub const BLACK: Color = Color::Ansi(AnsiColor::Black);
    /// ANSI red.
    pub const RED: Color = Color::Ansi(AnsiColor::Red);
    /// ANSI green.
    pub const GREEN: Color = Color::Ansi(AnsiColor::Green);
    /// ANSI yellow.
    pub const YELLOW: Color = Color::Ansi(AnsiColor::Yellow);
    /// ANSI blue.
    pub const BLUE: Color = Color::Ansi(AnsiColor::Blue);
    /// ANSI magenta.
    pub const MAGENTA: Color = Color::Ansi(AnsiColor::Magenta);
    /// ANSI cyan.
    pub const CYAN: Color = Color::Ansi(AnsiColor::Cyan);
    /// ANSI white.
    pub const WHITE: Color = Color::Ansi(AnsiColor::White);
    /// ANSI bright black (dark gray).
    pub const BRIGHT_BLACK: Color = Color::Ansi(AnsiColor::BrightBlack);
    /// ANSI bright red.
    pub const BRIGHT_RED: Color = Color::Ansi(AnsiColor::BrightRed);
    /// ANSI bright green.
    pub const BRIGHT_GREEN: Color = Color::Ansi(AnsiColor::BrightGreen);
    /// ANSI bright yellow.
    pub const BRIGHT_YELLOW: Color = Color::Ansi(AnsiColor::BrightYellow);
    /// ANSI bright blue.
    pub const BRIGHT_BLUE: Color = Color::Ansi(AnsiColor::BrightBlue);
    /// ANSI bright magenta.
    pub const BRIGHT_MAGENTA: Color = Color::Ansi(AnsiColor::BrightMagenta);
    /// ANSI bright cyan.
    pub const BRIGHT_CYAN: Color = Color::Ansi(AnsiColor::BrightCyan);
    /// ANSI bright white.
    pub const BRIGHT_WHITE: Color = Color::Ansi(AnsiColor::BrightWhite);

    // ── Named constructors ───────────────────────────────────────────────────

    /// Create a 24-bit RGB color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Create an Xterm 256-color palette index (0–255).
    pub const fn xterm(index: u8) -> Self {
        Color::Xterm(index)
    }

    /// Parse a CSS hex color string (`#RGB`, `#RRGGBB`, with or without the leading `#`).
    ///
    /// Returns `None` if the string is not a valid hex color.
    ///
    /// # Examples
    /// ```
    /// use spraypaint::Color;
    ///
    /// assert_eq!(Color::from_hex("#ff5733"), Some(Color::Rgb(255, 87, 51)));
    /// assert_eq!(Color::from_hex("#f00"),    Some(Color::Rgb(255, 0, 0)));
    /// assert_eq!(Color::from_hex("#xyz"),     None);
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                Some(Color::Rgb(r * 17, g * 17, b * 17))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color::Rgb(r, g, b))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_6digit() {
        assert_eq!(Color::from_hex("#ff5733"), Some(Color::Rgb(255, 87, 51)));
        assert_eq!(Color::from_hex("ff5733"), Some(Color::Rgb(255, 87, 51)));
    }

    #[test]
    fn hex_3digit() {
        assert_eq!(Color::from_hex("#f00"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(Color::from_hex("#fff"), Some(Color::Rgb(255, 255, 255)));
    }

    #[test]
    fn hex_invalid() {
        assert_eq!(Color::from_hex("zzzzzz"), None);
        assert_eq!(Color::from_hex("#12"), None);
    }

    #[test]
    fn ansi_codes() {
        assert_eq!(AnsiColor::Red.fg_code(), 31);
        assert_eq!(AnsiColor::Red.bg_code(), 41);
        assert_eq!(AnsiColor::BrightBlue.fg_code(), 94);
    }
}
