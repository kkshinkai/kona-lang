// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lexer --example token_list

use kona_lexer::{
    lexing::{tokenize, LexMode},
    token::TokenKind,
};

fn print_token_list(source: &str, lex_mode: LexMode) {
    let tokens = tokenize(source, LexMode::TokenAndTrivia);
    let mut pos = 0;

    println!("TokenList (lexMode = {lex_mode:?}) [");
    for token in tokens {
        let text = &source[pos..pos + token.len];
        if lex_mode == LexMode::TokenAndTrivia
            || !matches!(token.kind, TokenKind::Trivia(_))
        {
            println!("    {} {:?},", token.kind, text);
        }
        pos += token.len;
    }
    println!("]");
}

fn main() {
    print_token_list("fn x => x + 1", LexMode::TokenOnly);
    print_token_list("fn x => x + 1", LexMode::TokenAndTrivia);

    print_token_list(r#"
        let
            val sayHello = fn name => "Hello, " + name + "!";
            val name = "Kk Shinkai";
        in
            println (sayHello name);
        end
    "#, LexMode::TokenOnly);
}
