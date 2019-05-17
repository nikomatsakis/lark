pub trait InternKey: Sized {
    type Data: InternData<Key = Self>;

    fn lookup(self, db: &dyn Lookup<Self>) -> Self::Data {
        db.lookup(self)
    }
}

pub trait InternData: Sized {
    type Key: InternKey<Data = Self>;

    fn intern(self, db: &dyn Intern<Self>) -> Self::Key {
        db.intern(self)
    }
}

pub trait Intern<Data: InternData> {
    fn as_dyn(&self) -> &dyn Intern<Data>;

    fn intern(&self, data: Data) -> Data::Key;
}

impl<T, D> Intern<D> for &T
where
    T: ?Sized + Intern<D>,
    D: InternData,
{
    fn as_dyn(&self) -> &dyn Intern<D> {
        T::as_dyn(self)
    }

    fn intern(&self, data: D) -> D::Key {
        T::intern(self, data)
    }
}

pub trait Lookup<Key: InternKey> {
    fn as_dyn(&self) -> &dyn Lookup<Key>;

    fn lookup(&self, key: Key) -> Key::Data;
}

impl<T, K> Lookup<K> for &T
where
    T: ?Sized + Lookup<K>,
    K: InternKey,
{
    fn as_dyn(&self) -> &dyn Lookup<K> {
        T::as_dyn(self)
    }

    fn lookup(&self, key: K) -> K::Data {
        T::lookup(self, key)
    }
}

pub trait Interner<Key, Data>: Intern<Data> + Lookup<Key>
where
    Key: InternKey<Data = Data>,
    Data: InternData<Key = Key>,
{
    fn as_dyn(&self) -> &dyn Interner<Key, Data>;
}

#[macro_export]
macro_rules! intern_pair {
    ($key:ty, $data:ty) => {
        impl $crate::neo::InternKey for $key {
            type Data = $data;
        }

        impl $crate::neo::InternData for $data {
            type Key = $key;
        }
    };
}

#[macro_export]
macro_rules! interner_define {
    (impl[$($params:tt)*] Interner<$key_ty:ty, $data_ty:ty> for $base_ty:ty {
        fn intern($($intern_in:tt)*) -> $intern_out:ty {
            $($intern_body:tt)*
        }

        fn lookup($($lookup_in:tt)*) -> $lookup_out:ty {
            $($lookup_body:tt)*
        }
    }) => {
        impl<$($params)*> $crate::neo::Interner<$key_ty, $data_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Interner<$key_ty, $data_ty> {
                self
            }
        }

        impl<$($params)*> $crate::neo::Intern<$data_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Intern<$data_ty> {
                self
            }

            fn intern($($intern_in)*) -> $intern_out {
                $($intern_body)*
            }
        }

        impl<$($params)*> $crate::neo::Lookup<$key_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Lookup<$key_ty> {
                self
            }

            fn lookup($($lookup_in)*) -> $lookup_out {
                $($lookup_body)*
            }
        }
    }
}

#[macro_export]
macro_rules! interner_delegate {
    (impl[$($params:tt)*] Interner<$key_ty:ty, $data_ty:ty> for $base_ty:ty {
        $field:ident
    }) => {
        impl<$($params)*> $crate::neo::Interner<$key_ty, $data_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Interner<$key_ty, $data_ty> {
                self
            }
        }

        impl<$($params)*> $crate::neo::Intern<$data_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Intern<$data_ty> {
                self
            }

            fn intern(&self, data: $data_ty) -> $key_ty {
                self.$field.intern(data)
            }
        }

        impl<$($params)*> $crate::neo::Lookup<$key_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Lookup<$key_ty> {
                self
            }

            fn lookup(&self, data: $key_ty) -> $data_ty {
                self.$field.lookup(data)
            }
        }
    };
}
