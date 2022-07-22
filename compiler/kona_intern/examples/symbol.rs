// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --package kona_intern --example symbol

use kona_intern::symbol::Symbol;

fn main() {
    let alpha: &str = "alpha";
    let beta: String = "beta".to_string();
    let gamma = "gamma";

    let alpha_sym = Symbol::intern(alpha);
    let beta_sym = Symbol::intern(&beta);

    println!("\"alpha\" = {:?}", alpha_sym);
    println!("\"beta\" = {:?}", beta_sym);
    println!("\"gamma\" = {:?}", Symbol::intern(gamma));
}
