// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lexer --example highlight

use std::path::PathBuf;
use kona_lexer::{lexing::tokenize, token::{TokenKind, LitKind, TriviaKind}};

fn highlight(source: &str) {
    let tokens = tokenize(source);
    let mut pos = 0;

    for token in tokens {
        let text = &source[pos..pos + token.len];

        let color = match token.kind {
            kind if kind.is_keyword() => 34,
            TokenKind::Lit(lit_kind) => match lit_kind {
                LitKind::String { .. } => 32,
                LitKind::Int => 36,
                LitKind::Float => 36,
                LitKind::Bool => 33,
            },
            TokenKind::Invalid => 31,
            TokenKind::Trivia(TriviaKind::MultiLineComment { .. }
                             | TriviaKind::SingleLineComment) => 30,
            _ => 0,
        };

        print!("\x1b[{}m{}\x1b[0m", color, text);
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
