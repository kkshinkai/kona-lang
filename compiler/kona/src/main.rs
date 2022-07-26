// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{env, path::PathBuf};

use kona_source::source_map::SourceMap;

fn main() {
    // The command line parameters and tasks are simple, we don't need a driver
    // yet.
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_path] => println!("error: no input file"),
        [_path, file] => interpret(file),
        [_path, file, arg] => match arg.as_str() {
            // cargo run examples/hello.kona --lex
            "--lex" => lex(file),
            cmd => println!("error: unknown command '{}'", cmd),
        }
        _ => println!("{}", "error: wrong arguments"),
    }
}

fn interpret(file: &str) {
    println!("{}", file);
}

fn lex(file: &str) {
    let mut sm = SourceMap::new();
    let sf = sm.load_file(PathBuf::from(file))
        .expect(&format!("error: failed to load file '{}'", file));

    println!("{:?}", sf);
}
