// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

#[macro_use] extern crate lazy_static;

mod source_iter;
mod char_spec;
mod token;
mod lexing;

pub use lexing::tokenize;
pub use token::*;
