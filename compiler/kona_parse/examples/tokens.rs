// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_parse --example tokens

use std::path::PathBuf;

use kona_parse::lex::token_iter::TokenIter;
use kona_source::pos::Pos;

fn main() {
    let mut src_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src_file.push("examples/hello.kona");

    let error_msg = format!(
        "Failed to read example source file at \"{}\"",
        src_file.display(),
    );
    let src = std::fs::read_to_string(src_file)
        .expect(&error_msg);

    let mut ti = TokenIter::new(&src, Pos::from_usize(0));

    while let Some(token) = ti.eat() {
        println!("{:?}", token);
    }
}
