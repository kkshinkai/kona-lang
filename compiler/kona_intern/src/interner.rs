// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::symbol::Symbol;

#[derive(Default)]
pub struct Interner {
    storage: Vec<String>,
    names: HashMap<&'static str, Symbol>,
}

lazy_static! {
    pub(crate) static ref GLOBAL_INTERNER: Mutex<Interner> =
        Mutex::new(Interner::default());
}

impl Interner {
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&symbol) = self.names.get(string) {
            return symbol;
        }

        let symbol = Symbol::new(self.storage.len() as u32);
        self.storage.push(string.to_string());

        // SAFETY: We put the string into the interner, and never remove item
        // from it. It is safe to use `&'static str` here.
        let str: &'static str = unsafe {
            let str: &str = self.storage.last().unwrap().as_str();
            std::mem::transmute::<&str, &'static str>(str)
        };

        self.names.insert(str, symbol);
        symbol
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        &self.storage[symbol.id as usize]
    }
}
