//! Terminal color capability detection.
//!
//! Auto-detected on first access; override with [`set_color_level`].
//!
//! Checks (highest priority first): `NO_COLOR`, `FORCE_COLOR`, `COLORTERM`, `TERM`, TTY.

use std::io::IsTerminal as _;
use std::sync::atomic::{AtomicU8, Ordering};

/// Terminal color capability tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ColorLevel {
    /// Strip all ANSI codes.
    None = 0,
    /// 4-bit / 16 colors.
    Basic16 = 1,
    /// 8-bit / 256 colors.
    Xterm256 = 2,
    /// 24-bit truecolor.
    TrueColor = 3,
}

impl ColorLevel {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Basic16,
            2 => Self::Xterm256,
            _ => Self::TrueColor,
        }
    }
}

const UNSET: u8 = 255;
static LEVEL: AtomicU8 = AtomicU8::new(UNSET);

/// Current color level (auto-detects on first call).
pub fn color_level() -> ColorLevel {
    let v = LEVEL.load(Ordering::Relaxed);
    if v == UNSET {
        let detected = detect();
        LEVEL
            .compare_exchange(UNSET, detected as u8, Ordering::Relaxed, Ordering::Relaxed)
            .ok();
        ColorLevel::from_u8(LEVEL.load(Ordering::Relaxed))
    } else {
        ColorLevel::from_u8(v)
    }
}

/// Force a specific color level, bypassing detection.
pub fn set_color_level(level: ColorLevel) {
    LEVEL.store(level as u8, Ordering::Relaxed);
}

fn detect() -> ColorLevel {
    if std::env::var_os("NO_COLOR").is_some() {
        return ColorLevel::None;
    }

    if let Ok(val) = std::env::var("FORCE_COLOR") {
        return match val.trim() {
            "1" => ColorLevel::Basic16,
            "2" => ColorLevel::Xterm256,
            "3" => ColorLevel::TrueColor,
            _ => ColorLevel::Basic16,
        };
    }

    if let Ok(ct) = std::env::var("COLORTERM") {
        let ct = ct.to_ascii_lowercase();
        if ct == "truecolor" || ct == "24bit" {
            return ColorLevel::TrueColor;
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        let term = term.to_ascii_lowercase();
        if term.contains("256color") {
            return ColorLevel::Xterm256;
        }
        if term.starts_with("xterm")
            || term.starts_with("screen")
            || term.starts_with("tmux")
            || term.starts_with("vte")
            || term == "linux"
            || term == "rxvt"
        {
            return ColorLevel::Basic16;
        }
        if term == "dumb" {
            return ColorLevel::None;
        }
    }

    #[cfg(windows)]
    if std::env::var_os("WT_SESSION").is_some()
        || std::env::var_os("TERM_PROGRAM").map_or(false, |v| v == "vscode")
    {
        return ColorLevel::TrueColor;
    }

    if std::env::var_os("CI").is_some() {
        return ColorLevel::Basic16;
    }

    if std::io::stdout().is_terminal() {
        ColorLevel::Basic16
    } else {
        ColorLevel::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_ordering() {
        assert!(ColorLevel::None < ColorLevel::Basic16);
        assert!(ColorLevel::Basic16 < ColorLevel::Xterm256);
        assert!(ColorLevel::Xterm256 < ColorLevel::TrueColor);
    }

    #[test]
    fn round_trip_u8() {
        for v in 0u8..=3 {
            assert_eq!(ColorLevel::from_u8(v) as u8, v);
        }
    }
}
