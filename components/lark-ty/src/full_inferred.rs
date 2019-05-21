//! A type family where we have fully inferred all the types.
//! Permissions are partly erased (aliasing information lost). This is
//! the output of the `full_type_check` query.

use crate::BaseData;
use crate::Erased;
use crate::PermKind;
use crate::Placeholder;
use crate::ReprKind;
use crate::TypeFamily;
use crate::TypeInterners;
use lark_debug_derive::DebugWith;
use lark_debug_with::{DebugWith, FmtWithSpecialized};
use lark_intern::neo::InternData;
use lark_intern::neo::InternKey;
use std::fmt;

#[derive(Copy, Clone, Debug, DebugWith, PartialEq, Eq, Hash)]
pub struct FullInferred;

impl TypeFamily for FullInferred {
    type Repr = Erased; // FIXME
    type ReprData = Erased;
    type Perm = PermKind;
    type PermData = PermKind;
    type Base = Base;
    type BaseData = BaseData<FullInferred>;

    type Placeholder = Placeholder;

    fn own_perm(_tables: impl TypeInterners<Self>) -> PermKind {
        PermKind::Own
    }

    fn known_repr(_tables: impl TypeInterners<Self>, _repr_kind: ReprKind) -> Self::Repr {
        Erased
    }

    fn intern_base_data(tables: impl TypeInterners<Self>, base_data: BaseData<Self>) -> Self::Base {
        base_data.intern(&tables)
    }
}

lark_collections::index_type! {
    pub struct Base { .. }
}

lark_debug_with::debug_fallback_impl!(Base);

impl<Cx> FmtWithSpecialized<Cx> for Base
where
    Cx: TypeInterners<FullInferred>,
{
    fn fmt_with_specialized(&self, cx: &Cx, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup(cx).fmt_with(cx, fmt)
    }
}

lark_intern::intern_pair!(Base, BaseData<FullInferred>);
