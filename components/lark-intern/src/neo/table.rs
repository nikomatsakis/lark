use crate::neo::Interner;
use lark_collections::{FxIndexMap, U32Index};
use std::cell::RefCell;
use std::hash::Hash;

/// An "intern table" defines a single interner for
/// one key-value pair. They're meant to be grouped
/// into a larger `Interners` struct, a la
/// `crate::ty::TyInterners`, that define a series
/// of interners related to some particular area.
#[derive(Debug)]
pub struct InternTable<Key, Data>
where
    Key: Copy + U32Index,
    Data: Clone + Hash + Eq,
{
    map: RefCell<FxIndexMap<Data, ()>>,
    key: std::marker::PhantomData<Key>,
}

impl<Key, Data> Default for InternTable<Key, Data>
where
    Key: Copy + U32Index,
    Data: Clone + Hash + Eq,
{
    fn default() -> Self {
        InternTable {
            map: RefCell::new(FxIndexMap::default()),
            key: std::marker::PhantomData,
        }
    }
}

impl<Key, Data> Interner<Key, Data> for InternTable<Key, Data>
where
    Key: Copy + U32Index,
    Data: Clone + Hash + Eq,
{
    fn as_dyn(&self) -> &dyn Interner<Key, Data> {
        self
    }

    fn intern(&self, data: Data) -> Key {
        let mut map = self.map.borrow_mut();
        let entry = map.entry(data);
        let index = entry.index();
        entry.or_insert(());
        Key::from_usize(index)
    }

    fn lookup(&self, key: Key) -> Data {
        let map = self.map.borrow();
        match map.get_index(key.as_usize()) {
            Some((key, &())) => key.clone(),
            None => panic!("invalid intern index: `{:?}`", key),
        }
    }
}
