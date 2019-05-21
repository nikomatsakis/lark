//! A type family where we have fully inferred all the "base types" --
//! but all permissions are erased. This is the output of the
//! `base_type_check` query.

use crate::BaseData;
use crate::Erased;
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
pub struct BaseInferred;

impl TypeFamily for BaseInferred {
    type Repr = Erased;
    type ReprData = Erased;
    type Perm = Erased;
    type PermData = Erased;
    type Base = Base;
    type BaseData = BaseData<BaseInferred>;

    type Placeholder = Placeholder;

    fn own_perm(_tables: impl TypeInterners<BaseInferred>) -> Erased {
        Erased
    }

    fn known_repr(_tables: impl TypeInterners<BaseInferred>, _repr_kind: ReprKind) -> Self::Repr {
        Erased
    }

    fn intern_base_data(
        tables: impl TypeInterners<BaseInferred>,
        base_data: BaseData<Self>,
    ) -> Self::Base {
        base_data.intern(&tables)
    }
}

lark_collections::index_type! {
    pub struct Base { .. }
}

lark_debug_with::debug_fallback_impl!(Base);

impl<Cx> FmtWithSpecialized<Cx> for Base
where
    Cx: TypeInterners<BaseInferred>,
{
    fn fmt_with_specialized(&self, cx: &Cx, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup(cx).fmt_with(cx, fmt)
    }
}

lark_intern::intern_pair!(Base, BaseData<BaseInferred>);
