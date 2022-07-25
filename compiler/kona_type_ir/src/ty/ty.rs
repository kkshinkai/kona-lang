// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::cell::Cell;

use kona_memory::intern::symbol::Symbol;

pub struct TyVar<'tcx> {
    pub id: u32,
    pub data: Cell<Option<&'tcx Ty<'tcx>>>,
}

pub enum Ty<'tcx> {
    /// A type variable.
    Var(&'tcx TyVar<'tcx>),

    /// An n-ary type constructor.
    Con(TyCon, &'tcx [Ty<'tcx>]),
}

pub struct TyCon {
    pub name: Symbol,
    pub arity: usize,
}

pub enum TyScheme<'tcx> {
    Mono(&'tcx Ty<'tcx>),
    Poly(&'tcx [TyVar<'tcx>], &'tcx Ty<'tcx>),
}
