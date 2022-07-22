// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use crate::interner::GLOBAL_INTERNER;

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
