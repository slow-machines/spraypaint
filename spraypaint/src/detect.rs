//! Terminal color capability detection.
//!
//! On first access, spraypaint auto-detects the color level from the environment.
//! The level can be overridden at runtime with [`set_color_level`].
//!
//! # Environment Variables
//! - `NO_COLOR` (any value) → level 0 (no color)
//! - `FORCE_COLOR=1` → level 1 (basic 16)
//! - `FORCE_COLOR=2` → level 2 (256-color)
//! - `FORCE_COLOR=3` → level 3 (truecolor)
//! - `COLORTERM=truecolor` or `COLORTERM=24bit` → level 3
//! - `TERM=xterm-256color` etc. → level 2
//! - `TERM=xterm` etc. → level 1

use std::io::IsTerminal as _;
use std::sync::atomic::{AtomicU8, Ordering};

/// Color capability level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ColorLevel {
    /// No color output; all ANSI codes are stripped.
    None = 0,
    /// Basic 16-color (4-bit ANSI).
    Basic16 = 1,
    /// 256-color Xterm palette (8-bit).
    Xterm256 = 2,
    /// 24-bit truecolor RGB.
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

/// Sentinel value meaning "not yet initialized".
const UNSET: u8 = 255;

static LEVEL: AtomicU8 = AtomicU8::new(UNSET);

/// Return the current global color level, auto-detecting on first call.
///
/// Detection priority (highest wins):
/// 1. `NO_COLOR` env var → [`ColorLevel::None`]
/// 2. `FORCE_COLOR` env var (1/2/3) → forced level
/// 3. `COLORTERM=truecolor` / `COLORTERM=24bit` → [`ColorLevel::TrueColor`]
/// 4. `TERM`-based heuristics
/// 5. Actual TTY check on stdout (via `std::io::IsTerminal`)
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

/// Override the global color level, bypassing auto-detection.
///
/// Call this early in `main` if you want consistent behavior regardless of the
/// terminal environment.
///
/// # Example
/// ```rust,no_run
/// use spraypaint::detect::{set_color_level, ColorLevel};
/// set_color_level(ColorLevel::None); // disable color entirely
/// ```
pub fn set_color_level(level: ColorLevel) {
    LEVEL.store(level as u8, Ordering::Relaxed);
}

/// Detect the color level from the process environment.
fn detect() -> ColorLevel {
    // NO_COLOR takes priority over everything.
    if std::env::var_os("NO_COLOR").is_some() {
        return ColorLevel::None;
    }

    // FORCE_COLOR allows CI / pipelines to force a specific level.
    if let Ok(val) = std::env::var("FORCE_COLOR") {
        return match val.trim() {
            "1" => ColorLevel::Basic16,
            "2" => ColorLevel::Xterm256,
            "3" => ColorLevel::TrueColor,
            // "true" or non-numeric → basic color
            _ => ColorLevel::Basic16,
        };
    }

    // COLORTERM=truecolor or 24bit → truecolor
    if let Ok(ct) = std::env::var("COLORTERM") {
        let ct = ct.to_ascii_lowercase();
        if ct == "truecolor" || ct == "24bit" {
            return ColorLevel::TrueColor;
        }
    }

    // TERM-based heuristics
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

    // Windows Terminal and modern Windows console → truecolor
    #[cfg(windows)]
    if std::env::var_os("WT_SESSION").is_some()
        || std::env::var_os("TERM_PROGRAM").map_or(false, |v| v == "vscode")
    {
        return ColorLevel::TrueColor;
    }

    // On CI environments without a TTY, default to basic color so output is
    // readable when viewed in a browser log.
    if std::env::var_os("CI").is_some() {
        return ColorLevel::Basic16;
    }

    // Real TTY check: emit color only when stdout is actually a terminal.
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
