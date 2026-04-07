# spraypaint

<p align="center">
    <img width="250" height="250" alt="logo" src="https://i.postimg.cc/JhqpCWDh/logo.png" />
</p>

Terminal string styling for Rust.

```
cargo add spraypaint
```

---

## Why spraypaint?

| Feature | spraypaint | owo-colors | colored |
|---|---|---|---|
| `paint!` macro (prints directly) | yes | no | no |
| Tagged-template syntax | yes | no | no |
| `.paint()` terminal method | yes | no | no |
| RGB / hex colors | yes | yes | no |
| Gradient text | yes | no | no |
| Compile-time style validation | yes (macro) | no | no |
| Auto color detection | yes | yes | yes |
---

## Quick Start

```rust
use spraypaint::{paint, Colorize};

fn main() {
    // paint! prints directly -- no println! wrapper needed
    paint!("{red.bold Error:} something went wrong");
    paint!("Hello {green.italic world}!");

    // Extension trait: chain styles then call .paint()
    "Warning".yellow().on_red().italic().paint();

    // Compose styled values for loggers / format strings
    let status = "OK".green().bold();
    println!("Status: {status}");
}
```

---

## The `paint!` Macro

`paint!` is the primary API. It prints to stdout with a newline, just like `println!`,
but with inline styled spans.

### Syntax

```rust
use spraypaint::{paint, styled};

// {style.style text content}
paint!("{red.bold Error:} something went wrong");

// Multiple styles with dots
paint!("{green.italic.underline Success:} done!");

// Nested spans
paint!("{blue Welcome to {bold.underline spraypaint}!}");

// Embed Rust expressions with {expr}
let name = "Ferris";
let count = 42_u32;
paint!("Hello {green.bold {name}}, you have {cyan {count}} messages.");

// No newline (like print!)
paint!(inline, "{yellow Loading...}");

// To stderr
paint!(stderr, "{red.bold FATAL:} unrecoverable error");

// styled! returns a String instead of printing
let msg = styled!("{red.bold Error:} disk full");
log::error!("{msg}");
```

### Recognized style tokens

**Colors:** `black` `red` `green` `yellow` `blue` `magenta` `cyan` `white`
and their `bright_` variants.

**Backgrounds:** prefix any color with `on_` -- `on_red`, `on_bright_blue`, etc.

**Attributes:** `bold` `dim` `italic` `underline` `blink` `blink_fast` `reverse` `hidden` `strikethrough`

**Invalid tokens produce a compile error:**
```
error: unknown style `blod` in paint! template
       hint: valid attributes are bold, dim, italic, underline, strikethrough, reverse
```

---

## Extension Trait

Import `Colorize` to style any `Display` value:

```rust
use spraypaint::Colorize;

// Colors
"red text".red().paint();
"blue text".blue().paint();
"custom".rgb(255, 87, 51).paint();
"hex".hex("#6c5ce7").paint();
"xterm index".xterm(202).paint();

// Backgrounds
"warning".black().on_yellow().paint();
"danger".white().on_red().paint();

// Attributes
"bold".bold().paint();
"italic".italic().paint();
"underline".underline().paint();
"dim".dim().paint();
"strikethrough".strikethrough().paint();

// Chaining (flat -- no nesting penalty)
"error".red().bold().underline().paint();

// Without newline
"loading".yellow().paint_inline();

// To stderr
"fatal".red().bold().paint_err();
```

Works on any `Display` type:

```rust
42_i32.green().bold().paint();
3.14_f64.cyan().paint();
true.yellow().paint();
```

---

## Reusable Styles

```rust
use spraypaint::Style;

let error = Style::new().red().bold().underline();
let warn   = Style::new().yellow().bold();
let info   = Style::new().cyan();

error.apply("CRITICAL").paint();
warn.apply("WARNING").paint();
info.apply("INFO").paint();

// Merge styles
let loud_error = error.merge(Style::new().blink());
loud_error.apply("LOUD ERROR").paint();
```

---

## Gradients

```rust
use spraypaint::{Color, Colorize};

// Two-stop gradient
"Red to Cyan".gradient(Color::RED, Color::CYAN).paint();

// Multi-stop (full rainbow)
let stops = vec![
    Color::rgb(255, 0, 0),
    Color::rgb(255, 165, 0),
    Color::rgb(255, 255, 0),
    Color::rgb(0, 255, 0),
    Color::rgb(0, 0, 255),
];
"The quick brown fox".gradient_multi(stops).paint();

// Hex stops
let from = Color::from_hex("#6c5ce7").unwrap();
let to   = Color::from_hex("#fd79a8").unwrap();
"Purple to Pink".gradient(from, to).paint();
```

---

## Color Detection

spraypaint auto-detects terminal color support on first use.

| Environment Variable | Effect |
|---|---|
| `NO_COLOR=1` | Disable all color (level 0) |
| `FORCE_COLOR=1` | Force basic 16-color |
| `FORCE_COLOR=2` | Force 256-color |
| `FORCE_COLOR=3` | Force truecolor |
| `COLORTERM=truecolor` | Enable truecolor |
| `TERM=xterm-256color` | Enable 256-color |

Override programmatically:

```rust
use spraypaint::detect::{set_color_level, ColorLevel};

set_color_level(ColorLevel::TrueColor);
// or
set_color_level(ColorLevel::None); // disable color
```

Colors automatically downgrade when the terminal doesn't support them:
- TrueColor → Xterm256 → Basic16 → no color

---

## Color Reference

```rust
use spraypaint::Color;

Color::RED          // named constant (ANSI)
Color::rgb(r, g, b) // 24-bit truecolor
Color::xterm(202)   // xterm 256-color index
Color::from_hex("#ff5733")  // parse CSS hex
Color::from_hex("#f00")     // 3-digit shorthand
```

---

## Examples

```
cargo run --example basic        # colors and attributes
cargo run --example paint_macro  # paint! macro features
cargo run --example gradient     # gradient text
cargo run --example detect       # color level detection
```
