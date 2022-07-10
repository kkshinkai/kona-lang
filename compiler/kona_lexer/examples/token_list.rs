// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lexer --example token_list

use kona_lexer::{
    lexing::tokenize,
};

fn print_token_list(source: &str) {
    let tokens = tokenize(source);
    let mut pos = 0;

    for token in tokens {
        let text = &source[pos..pos + token.len];
        println!("({} {:?})", token.kind, text);
        pos += token.len;
    }
}

fn main() {
    print_token_list("fn x => x + 1")
}
