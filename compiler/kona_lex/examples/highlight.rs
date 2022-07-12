// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lex --example highlight

use std::path::PathBuf;
use kona_lex::{lexing::tokenize, token::{TokenKind, LitKind, TriviaKind, IdentKind}};

/// Gets 256-color mode color (`ESC[38;5;#m`) for a token kind.
fn get_color(kind: TokenKind) -> Option<u8> {
    match kind {
        _ if kind.is_alphanumeric_keyword() => Some(125),
        _ if kind.is_symbolic_keyword() => Some(18),
        _ if kind.is_punct() => Some(219),
        _ if kind.is_comment() => Some(246),
        TokenKind::Ident(IdentKind::Alphanumeric) => Some(98),
        TokenKind::Ident(IdentKind::Symbolic) => Some(75),
        TokenKind::Lit(lit_kind) => match lit_kind {
            LitKind::Int => Some(155),
            LitKind::Float => Some(155),
            LitKind::String { .. } => Some(178),
            LitKind::Bool => Some(202),
        },
        _ => None,
    }
}

fn highlight(source: &str) {
    let tokens = tokenize(source);
    let mut pos = 0;

    for token in tokens {
        let text = &source[pos..pos + token.len];

        if let Some(color) = get_color(token.kind) {
            print!("\x1b[38;5;{}m{}\x1b[0m", color, text);
        } else {
            print!("{}", text);
        }

        pos += token.len;
    }
    println!("");
}

fn main() {
    let mut src_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src_file.push("examples/hello.kona");

    let error_msg = format!(
        "Failed to read example source file at \"{}\"",
        src_file.display(),
    );
    let src = std::fs::read_to_string(src_file)
        .expect(&error_msg);

    highlight(&src);
}
