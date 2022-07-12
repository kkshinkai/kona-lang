// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lex --example token_list

use std::path::PathBuf;

use kona_lex::lexing::tokenize;

fn print_token_list(source: &str) {
    let tokens = tokenize(source);
    let mut pos = 0;

    println!("TokenList [");
    for token in tokens {
        let text = &source[pos..pos + token.len];
        println!("    {} {:?},", token.kind, text);
        pos += token.len;
    }
    println!("]");
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

    print_token_list(&src);
}
