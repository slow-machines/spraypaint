//! Demonstrates gradient text with two-stop and multi-stop gradients.
//!
//! Run with: cargo run --example gradient

use spraypaint::{Color, Colorize};

fn main() {
    // Two-stop gradients
    "Red to Cyan".gradient(Color::RED, Color::CYAN).paint();
    "Blue to Green".gradient(Color::BLUE, Color::GREEN).paint();
    "Magenta to Yellow"
        .gradient(Color::MAGENTA, Color::YELLOW)
        .paint();

    println!();

    // Multi-stop: full RGB rainbow
    let rainbow_stops = vec![
        Color::rgb(255, 0, 0),   // red
        Color::rgb(255, 165, 0), // orange
        Color::rgb(255, 255, 0), // yellow
        Color::rgb(0, 255, 0),   // green
        Color::rgb(0, 0, 255),   // blue
        Color::rgb(75, 0, 130),  // indigo
        Color::rgb(148, 0, 211), // violet
    ];
    "The quick brown fox jumps over the lazy dog"
        .gradient_multi(rainbow_stops)
        .paint();

    println!();

    // Gradient from hex colors
    let hex_from = Color::from_hex("#6c5ce7").unwrap(); // purple
    let hex_to = Color::from_hex("#fd79a8").unwrap(); // pink
    "Purple to Pink gradient".gradient(hex_from, hex_to).paint();

    println!();

    // Gradient with additional attributes
    use spraypaint::Style;
    "Bold gradient"
        .gradient(Color::CYAN, Color::MAGENTA)
        .with_style(Style::new().bold())
        .paint();

    // Short strings still work
    "Hi".gradient(Color::RED, Color::BLUE).paint();
    "X".gradient(Color::GREEN, Color::YELLOW).paint();
}
