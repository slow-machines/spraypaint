//! Procedural macros for the `spraypaint` crate.
//!
//! This crate is not intended to be used directly; import from `spraypaint` instead.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Token,
};

// ── Public macro entrypoints ──────────────────────────────────────────────────

/// Print styled text to stdout with a trailing newline.
///
/// # Syntax
/// ```text
/// paint!("template string")
/// paint!(inline, "template string")   // no trailing newline
/// paint!(stderr, "template string")   // print to stderr with newline
/// ```
///
/// # Template Format
/// ```text
/// paint!("{red.bold Error:} something went wrong");
/// paint!("Hello {green.italic world}!");
/// paint!("{blue Welcome to {bold.underline spraypaint}}");
///
/// let name = "world";
/// paint!("Hello {green.bold {name}}!");
/// ```
#[proc_macro]
pub fn paint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PaintInput);
    let parts = match parse_template(&input.template, input.template_span) {
        Ok(p) => p,
        Err(e) => return e.into_compile_error().into(),
    };
    let exprs = parts_to_exprs(&parts);

    match input.mode {
        PrintMode::Stdout => quote! {
            {
                use ::std::io::Write as _;
                #( ::std::print!("{}", #exprs); )*
                ::std::println!();
            }
        },
        PrintMode::Inline => quote! {
            {
                use ::std::io::Write as _;
                #( ::std::print!("{}", #exprs); )*
                let _ = ::std::io::stdout().flush();
            }
        },
        PrintMode::Stderr => quote! {
            {
                #( ::std::eprint!("{}", #exprs); )*
                ::std::eprintln!();
            }
        },
    }
    .into()
}

/// Return an owned `String` with ANSI styling applied (does not print).
///
/// Use this when you need to pass styled text to a logger, format string, etc.
///
/// # Example
/// ```rust,ignore
/// use spraypaint::styled;
///
/// let msg = styled!("{red.bold Error:} something went wrong");
/// eprintln!("{msg}");
/// ```
#[proc_macro]
pub fn styled(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let span = lit.span();
    let template = lit.value();

    let parts = match parse_template(&template, span) {
        Ok(p) => p,
        Err(e) => return e.into_compile_error().into(),
    };
    let exprs = parts_to_exprs(&parts);

    quote! {
        {
            use ::std::fmt::Write as _;
            let mut __buf = ::std::string::String::new();
            #( ::std::write!(__buf, "{}", #exprs)
                .expect("fmt::Write to String is infallible"); )*
            __buf
        }
    }
    .into()
}

// ── Input structs ─────────────────────────────────────────────────────────────

enum PrintMode {
    Stdout,
    Inline,
    Stderr,
}

struct PaintInput {
    mode: PrintMode,
    template: String,
    template_span: Span,
}

impl Parse for PaintInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) && !input.peek2(Token![,]) {
            // Single ident with no comma: must be the template (treat as error below
            // if it's not a string literal). Fall through.
        }
        if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;
            let _: Token![,] = input.parse()?;
            let lit: LitStr = input.parse()?;
            let mode = match ident.to_string().as_str() {
                "inline" => PrintMode::Inline,
                "stderr" => PrintMode::Stderr,
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown paint! mode `{other}`; expected `inline` or `stderr`"),
                    ))
                }
            };
            return Ok(PaintInput {
                mode,
                template: lit.value(),
                template_span: lit.span(),
            });
        }
        let lit: LitStr = input.parse()?;
        Ok(PaintInput {
            mode: PrintMode::Stdout,
            template: lit.value(),
            template_span: lit.span(),
        })
    }
}

// ── Template AST ──────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Part {
    /// A plain string literal fragment.
    Literal(String),
    /// A `{style1.style2 content}` block.
    Styled {
        styles: Vec<String>,
        inner: Vec<Part>,
    },
    /// A `{expr}` interpolation (arbitrary Rust expression).
    Expr(String),
}

// ── Template parser ───────────────────────────────────────────────────────────

fn parse_template(template: &str, span: Span) -> syn::Result<Vec<Part>> {
    let chars: Vec<char> = template.chars().collect();
    let mut parser = Parser {
        chars,
        pos: 0,
        span,
    };
    parser.parse_sequence(false)
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
    span: Span,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    /// Parse a sequence of parts until EOF or (if `stop=true`) a closing `}`.
    fn parse_sequence(&mut self, stop_at_close: bool) -> syn::Result<Vec<Part>> {
        let mut parts = Vec::new();
        let mut literal = String::new();

        loop {
            match self.peek() {
                None => break,
                Some('}') if stop_at_close => {
                    self.advance();
                    break;
                }
                Some('{') => {
                    if !literal.is_empty() {
                        parts.push(Part::Literal(std::mem::take(&mut literal)));
                    }
                    self.advance(); // consume '{'
                    parts.push(self.parse_brace()?);
                }
                Some(c) => {
                    literal.push(c);
                    self.advance();
                }
            }
        }

        if !literal.is_empty() {
            parts.push(Part::Literal(literal));
        }
        Ok(parts)
    }

    /// Parse the content after an opening `{`.
    fn parse_brace(&mut self) -> syn::Result<Part> {
        // Peek ahead to determine whether this is a style spec or a raw expression.
        // Strategy: read the first "word" (up to whitespace or end of brace group).
        let save = self.pos;
        let first_word = self.read_word();

        if !first_word.is_empty() && looks_like_style_spec(&first_word) {
            // Parse style spec tokens.
            let styles = parse_style_spec(&first_word, self.span)?;
            // Skip optional single space between spec and content.
            if self.peek() == Some(' ') {
                self.advance();
            }
            let inner = self.parse_sequence(true)?;
            Ok(Part::Styled { styles, inner })
        } else {
            // Raw Rust expression: restore position and consume until matching `}`.
            self.pos = save;
            let expr = self.read_until_close_brace()?;
            Ok(Part::Expr(expr.trim().to_string()))
        }
    }

    /// Read contiguous non-whitespace, non-brace characters (the style spec).
    fn read_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '{' || c == '}' {
                break;
            }
            word.push(c);
            self.advance();
        }
        word
    }

    /// Consume until the matching `}` (handling nested braces), return the content.
    fn read_until_close_brace(&mut self) -> syn::Result<String> {
        let mut s = String::new();
        let mut depth = 1usize;
        loop {
            match self.advance() {
                None => {
                    return Err(syn::Error::new(
                        self.span,
                        "unclosed `{` in paint! template",
                    ));
                }
                Some('{') => {
                    depth += 1;
                    s.push('{');
                }
                Some('}') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    s.push('}');
                }
                Some(c) => s.push(c),
            }
        }
        Ok(s)
    }
}

// ── Style spec helpers ────────────────────────────────────────────────────────

/// Return true if every dot-separated token in `spec` is a known style name.
fn looks_like_style_spec(spec: &str) -> bool {
    spec.split('.').all(|t| {
        let t = t.trim();
        is_known_style(t) || t.starts_with("rgb(") || t.starts_with("hex(")
    })
}

fn is_known_style(token: &str) -> bool {
    matches!(
        token,
        "black"
            | "red"
            | "green"
            | "yellow"
            | "blue"
            | "magenta"
            | "cyan"
            | "white"
            | "bright_black"
            | "bright_red"
            | "bright_green"
            | "bright_yellow"
            | "bright_blue"
            | "bright_magenta"
            | "bright_cyan"
            | "bright_white"
            | "on_black"
            | "on_red"
            | "on_green"
            | "on_yellow"
            | "on_blue"
            | "on_magenta"
            | "on_cyan"
            | "on_white"
            | "on_bright_black"
            | "on_bright_red"
            | "on_bright_green"
            | "on_bright_yellow"
            | "on_bright_blue"
            | "on_bright_magenta"
            | "on_bright_cyan"
            | "on_bright_white"
            | "bold"
            | "dim"
            | "italic"
            | "underline"
            | "blink"
            | "blink_fast"
            | "reverse"
            | "hidden"
            | "strikethrough"
    )
}

fn parse_style_spec(spec: &str, span: Span) -> syn::Result<Vec<String>> {
    let mut result = Vec::new();
    for token in spec.split('.') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if is_known_style(token) || token.starts_with("rgb(") || token.starts_with("hex(") {
            result.push(token.to_string());
        } else {
            return Err(syn::Error::new(
                span,
                format!(
                    "unknown style `{token}` in paint! template\n\
                     hint: valid colors are red, green, blue, yellow, magenta, cyan, white, black\n\
                     hint: valid attributes are bold, dim, italic, underline, strikethrough, reverse"
                ),
            ));
        }
    }
    Ok(result)
}

// ── Code generation ───────────────────────────────────────────────────────────

fn parts_to_exprs(parts: &[Part]) -> Vec<proc_macro2::TokenStream> {
    parts.iter().map(part_to_expr).collect()
}

fn part_to_expr(part: &Part) -> proc_macro2::TokenStream {
    match part {
        Part::Literal(s) => {
            quote! { #s }
        }

        Part::Expr(e) => {
            let tokens: proc_macro2::TokenStream = match e.parse() {
                Ok(t) => t,
                Err(lex_err) => {
                    let msg =
                        format!("invalid expression `{e}` in paint!/styled! template: {lex_err}");
                    return quote! { { compile_error!(#msg); "" } };
                }
            };
            quote! { &::std::format!("{}", #tokens) }
        }

        Part::Styled { styles, inner } => {
            let inner_exprs = parts_to_exprs(inner);
            let method_chain = build_method_chain(styles);

            // Collect inner parts into an owned String, then apply the style chain.
            // `String` implements `Colorize`, so chaining methods returns `Styled<String>`,
            // which implements `Display` -- safe to pass to `print!`.
            quote! {
                {
                    use ::std::fmt::Write as _;
                    use ::spraypaint::Colorize as _;
                    let mut __inner = ::std::string::String::new();
                    #( ::std::write!(__inner, "{}", #inner_exprs)
                        .expect("fmt::Write to String is infallible"); )*
                    __inner #method_chain
                }
            }
        }
    }
}

/// Build a dot-chained method call sequence, e.g. `.red().bold().italic()`.
fn build_method_chain(styles: &[String]) -> proc_macro2::TokenStream {
    let mut chain = quote! {};
    for style in styles {
        if let Some(inner) = style.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    chain = quote! { #chain .rgb(#r, #g, #b) };
                    continue;
                }
            }
        }
        if let Some(inner) = style.strip_prefix("hex(").and_then(|s| s.strip_suffix(')')) {
            chain = quote! { #chain .hex(#inner) };
            continue;
        }
        let ident = proc_macro2::Ident::new(style, Span::call_site());
        chain = quote! { #chain .#ident() };
    }
    chain
}
