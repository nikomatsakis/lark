//! Definition of a type family + type-checker methods for doing "base
//! only" inference. This is inference where we ignore permissions and
//! representations and focus only on the base types.

use lark_debug_derive::DebugWith;
use lark_intern::Intern;
use lark_ty::BaseData;
use lark_ty::Erased;
use lark_ty::InferVarOr;
use lark_ty::PermKind;
use lark_ty::Placeholder;
use lark_ty::ReprKind;
use lark_ty::TypeFamily;
use lark_ty::TypeInterners;
use std::cell::RefCell;

crate mod apply_perm;

crate mod analysis;

/// Defines the `Base` type that represents base types.
crate mod base;
use base::Base;

crate mod constraint;

/// Defines the `Perm` type that represents permissions.
crate mod perm;
use perm::Perm;
use perm::PermData;

crate mod query_definition;

mod resolve_to_full_inferred;

/// Implements the `TypeCheckerFamilyDependentExt` methods along with substitution.
crate mod type_checker;

/// Type family for "base inference" -- inferring just the base types.
#[derive(Copy, Clone, Debug, DebugWith, PartialEq, Eq, Hash)]
crate struct FullInference;

impl TypeFamily for FullInference {
    type Repr = Erased;
    type ReprData = Erased;
    type Perm = perm::Perm;
    type PermData = PermData;
    type Base = Base;
    type BaseData = InferVarOr<BaseData<FullInference>>;
    type Placeholder = Placeholder;

    fn own_perm(tables: &dyn TypeInterners<Self>) -> Self::Perm {
        tables.intern_perm(PermData::Known(PermKind::Own))
    }

    fn known_repr(_tables: &dyn TypeInterners<Self>, _repr_kind: ReprKind) -> Self::Repr {
        Erased
    }

    fn intern_base_data(tables: &dyn TypeInterners<Self>, base_data: BaseData<Self>) -> Self::Base {
        tables.intern_base(InferVarOr::Known(base_data))
    }
}

pub struct FullInferenceTables {
    base_table: RefCell<lark_intern::InternTable<Base, InferVarOr<BaseData<FullInference>>>>,
    perm_table: RefCell<lark_intern::InternTable<perm::Perm, PermData>>,
}

impl TypeInterners<FullInference> for FullInferenceTables {
    fn as_dyn(&self) -> &dyn TypeInterners<FullInference> {
        self
    }

    fn intern_repr(&self, repr: Erased) -> Erased {
        repr
    }

    fn lookup_repr(&self, repr: Erased) -> Erased {
        repr
    }

    fn intern_perm(&self, value: PermData) -> perm::Perm {
        self.perm_table.borrow_mut().intern(value)
    }

    fn lookup_perm(&self, value: perm::Perm) -> PermData {
        self.perm_table.borrow().get(value)
    }

    fn intern_base(&self, base: InferVarOr<BaseData<FullInference>>) -> Base {
        self.table.borrow_mut().intern(base)
    }

    fn lookup_base(&self, base: Base) -> InferVarOr<BaseData<FullInference>> {
        self.table.borrow().get(base)
    }
}
