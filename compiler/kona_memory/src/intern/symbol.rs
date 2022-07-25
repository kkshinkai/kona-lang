// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub(crate) id: u32,
}

impl Symbol {
    pub(crate) fn new(id: u32) -> Symbol {
        Symbol { id }
    }

    pub fn intern(string: &str) -> Symbol {
        GLOBAL_INTERNER.lock().unwrap().intern(string)
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: The lifetime of the return value is the same as `&self`, but
        // actually tied to the lifetime of the underlying interner. Interners
        // are long-lived, this cast is safe.
        unsafe {
            std::mem::transmute::<&str, &str>(
                GLOBAL_INTERNER.lock().unwrap().get(*self),
            )
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_INTERNER: Mutex<Interner> =
        Mutex::new(Interner::default());
}

#[derive(Default)]
struct Interner {
    storage: Vec<String>,
    names: HashMap<&'static str, Symbol>,
}


impl Interner {
    fn intern(&mut self, string: &str) -> Symbol {
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

    fn get(&self, symbol: Symbol) -> &str {
        &self.storage[symbol.id as usize]
    }
}
