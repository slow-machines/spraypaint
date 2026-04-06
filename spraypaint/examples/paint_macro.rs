//! Demonstrates the `paint!` macro with tagged-template syntax.
//!
//! Run with: cargo run --example paint_macro

use spraypaint::{paint, styled};

fn main() {
    // Simplest form: style a span directly
    paint!("{red Error:} something went wrong");
    paint!("{green Success:} operation completed");
    paint!("{yellow Warning:} low disk space");

    println!();

    // Multiple styles separated by dots
    paint!("{red.bold FATAL:} unrecoverable error");
    paint!("{green.italic hint:} try using --help");
    paint!("{blue.underline link:} https://example.com");

    println!();

    // Nested styles
    paint!("{blue Welcome to {bold.underline spraypaint}!}");
    paint!("{dim (c) 2024 {italic spraypaint contributors}}");

    println!();

    // Dynamic Rust expressions embedded with { }
    let name = "Ferris";
    let count = 42_u32;
    paint!("Hello {green.bold {name}}, you have {cyan {count}} messages.");

    let version = env!("CARGO_PKG_VERSION");
    paint!("spraypaint v{yellow {version}}");

    println!();

    // print! variant (no newline)
    paint!(inline, "{yellow Loading}");
    paint!(inline, ".");
    paint!(inline, ".");
    paint!(inline, ".");
    println!(" done");

    println!();

    // stderr variant
    paint!(stderr, "{red.bold [ERROR]} this goes to stderr");

    println!();

    // styled! -- returns a String, doesn't print
    let msg = styled!("{red.bold Error:} disk full");
    // Can be used in normal format strings, log macros, etc.
    eprintln!("logged: {msg}");

    let greeting = styled!("Hello {green {name}}!");
    let len = greeting.len();
    println!("Greeting ({len} chars including ANSI): {greeting}");
}
