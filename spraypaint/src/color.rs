//! Color types: ANSI 16, Xterm 256, and 24-bit RGB.

/// Standard 16-color ANSI palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColor {
    /// SGR 30/40
    Black,
    /// SGR 31/41
    Red,
    /// SGR 32/42
    Green,
    /// SGR 33/43
    Yellow,
    /// SGR 34/44
    Blue,
    /// SGR 35/45
    Magenta,
    /// SGR 36/46
    Cyan,
    /// SGR 37/47
    White,
    /// SGR 90/100
    BrightBlack,
    /// SGR 91/101
    BrightRed,
    /// SGR 92/102
    BrightGreen,
    /// SGR 93/103
    BrightYellow,
    /// SGR 94/104
    BrightBlue,
    /// SGR 95/105
    BrightMagenta,
    /// SGR 96/106
    BrightCyan,
    /// SGR 97/107
    BrightWhite,
}

impl AnsiColor {
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

    pub(crate) fn bg_code(self) -> u8 {
        self.fg_code() + 10
    }
}

/// Terminal color: ANSI 16, Xterm 256, or 24-bit RGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Standard 16-color.
    Ansi(AnsiColor),
    /// Xterm palette index (0–255).
    Xterm(u8),
    /// 24-bit truecolor.
    Rgb(u8, u8, u8),
}

impl Color {
    #[allow(missing_docs)]
    pub const BLACK: Color = Color::Ansi(AnsiColor::Black);
    #[allow(missing_docs)]
    pub const RED: Color = Color::Ansi(AnsiColor::Red);
    #[allow(missing_docs)]
    pub const GREEN: Color = Color::Ansi(AnsiColor::Green);
    #[allow(missing_docs)]
    pub const YELLOW: Color = Color::Ansi(AnsiColor::Yellow);
    #[allow(missing_docs)]
    pub const BLUE: Color = Color::Ansi(AnsiColor::Blue);
    #[allow(missing_docs)]
    pub const MAGENTA: Color = Color::Ansi(AnsiColor::Magenta);
    #[allow(missing_docs)]
    pub const CYAN: Color = Color::Ansi(AnsiColor::Cyan);
    #[allow(missing_docs)]
    pub const WHITE: Color = Color::Ansi(AnsiColor::White);
    #[allow(missing_docs)]
    pub const BRIGHT_BLACK: Color = Color::Ansi(AnsiColor::BrightBlack);
    #[allow(missing_docs)]
    pub const BRIGHT_RED: Color = Color::Ansi(AnsiColor::BrightRed);
    #[allow(missing_docs)]
    pub const BRIGHT_GREEN: Color = Color::Ansi(AnsiColor::BrightGreen);
    #[allow(missing_docs)]
    pub const BRIGHT_YELLOW: Color = Color::Ansi(AnsiColor::BrightYellow);
    #[allow(missing_docs)]
    pub const BRIGHT_BLUE: Color = Color::Ansi(AnsiColor::BrightBlue);
    #[allow(missing_docs)]
    pub const BRIGHT_MAGENTA: Color = Color::Ansi(AnsiColor::BrightMagenta);
    #[allow(missing_docs)]
    pub const BRIGHT_CYAN: Color = Color::Ansi(AnsiColor::BrightCyan);
    #[allow(missing_docs)]
    pub const BRIGHT_WHITE: Color = Color::Ansi(AnsiColor::BrightWhite);

    /// 24-bit RGB.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Xterm 256-color palette index.
    pub const fn xterm(index: u8) -> Self {
        Color::Xterm(index)
    }

    /// Parse `#RGB` or `#RRGGBB` (leading `#` optional). Returns `None` on bad input.
    ///
    /// ```
    /// use spraypaint::Color;
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
