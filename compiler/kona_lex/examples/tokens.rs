// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_lex --example tokens

use std::path::PathBuf;

use kona_lex::lexer::tokenize;
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

    tokenize(&src, Pos::from_usize(0)).for_each(|token|
        println!("{:?}", token)
    );
}
