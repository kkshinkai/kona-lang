// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lexer --example token_list

use kona_lexer::{
    lexing::{tokenize, LexMode},
};

fn print_token_list(source: &str, lex_mode: LexMode) {
    let tokens = tokenize(source, lex_mode);
    let mut pos = 0;

    println!("TokenList (lexMode = {lex_mode:?}) [");
    for token in tokens {
        let text = &source[pos..pos + token.len];
        println!("    {} {:?},", token.kind, text);
        pos += token.len;
    }
    println!("]");
}

fn main() {
    print_token_list("fn x => x + 1", LexMode::TokenAndTrivia);
}
