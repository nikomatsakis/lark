//! A type family where we preserve what the user wrote in all cases.
//! We do not support inference and bases and things may map to bound
//! variables from generic declarations.

use crate::BaseData;
use crate::BoundVar;
use crate::BoundVarOr;
use crate::ReprKind;
use crate::TypeFamily;
use crate::TypeInterners;
use crate::TypeLookup;
use lark_debug_derive::DebugWith;
use lark_debug_with::{DebugWith, FmtWithSpecialized};
use std::fmt;

#[derive(Copy, Clone, Debug, DebugWith, PartialEq, Eq, Hash)]
pub struct Declaration;

impl TypeFamily for Declaration {
    type Repr = ReprKind;
    type ReprData = ReprKind;
    type Perm = Perm;
    type PermData = DeclaredPermKind;
    type Base = Base;
    type BaseData = BoundVarOr<BaseData<Declaration>>;
    type Placeholder = !;

    fn own_perm(tables: &dyn TypeInterners<Self>) -> Self::Perm {
        tables.intern_perm(DeclaredPermKind::Own)
    }

    fn known_repr(_tables: &dyn TypeInterners<Self>, repr_kind: ReprKind) -> ReprKind {
        repr_kind
    }

    fn intern_base_data(tables: &dyn TypeInterners<Self>, base_data: BaseData<Self>) -> Self::Base {
        tables.intern_base(BoundVarOr::Known(base_data))
    }
}

impl Declaration {
    pub fn intern_bound_var(db: impl TypeInterners<Declaration>, bv: BoundVar) -> Base {
        let bv: BoundVarOr<BaseData<Declaration>> = BoundVarOr::BoundVar(bv);
        db.intern_base(bv)
    }
}

lark_collections::index_type! {
    pub struct Base { .. }
}

lark_debug_with::debug_fallback_impl!(Base);

impl<Cx> FmtWithSpecialized<Cx> for Base
where
    Cx: TypeInterners<Declaration>,
{
    fn fmt_with_specialized(&self, cx: &Cx, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup(cx).fmt_with(cx, fmt)
    }
}

lark_collections::index_type! {
    pub struct Perm { .. }
}

lark_debug_with::debug_fallback_impl!(Perm);

impl<Cx> FmtWithSpecialized<Cx> for Perm
where
    Cx: TypeInterners<Declaration>,
{
    fn fmt_with_specialized(&self, cx: &Cx, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup(cx).fmt_with(cx, fmt)
    }
}

/// For now, we only support `own T` in declarations.
#[derive(Copy, Clone, Debug, DebugWith, PartialEq, Eq, Hash)]
pub enum DeclaredPermKind {
    Own,
}

impl TypeLookup<Declaration> for Base {
    type Data = BoundVarOr<BaseData<Declaration>>;

    fn lookup(self, db: &dyn TypeInterners<Declaration>) -> Self::Data {
        db.lookup_base(self)
    }
}

impl TypeLookup<Declaration> for Perm {
    type Data = DeclaredPermKind;

    fn lookup(self, db: &dyn TypeInterners<Declaration>) -> Self::Data {
        db.lookup_perm(self)
    }
}
