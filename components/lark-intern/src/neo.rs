pub mod table;

pub trait InternKey: Sized {
    type Data: InternData<Key = Self>;

    fn lookup(self, db: &dyn Interner<Self, Self::Data>) -> Self::Data {
        db.lookup(self)
    }
}

pub trait InternData: Sized {
    type Key: InternKey<Data = Self>;

    fn intern(self, db: &dyn Interner<Self::Key, Self>) -> Self::Key {
        db.intern(self)
    }
}

pub trait Interner<Key, Data> {
    fn as_dyn(&self) -> &dyn Interner<Key, Data>;
    fn intern(&self, data: Data) -> Key;
    fn lookup(&self, key: Key) -> Data;
}

impl<T, Key, Data> Interner<Key, Data> for &T
where
    T: ?Sized + Interner<Key, Data>,
{
    fn as_dyn(&self) -> &dyn Interner<Key, Data> {
        T::as_dyn(self)
    }

    fn intern(&self, data: Data) -> Key {
        T::intern(self, data)
    }

    fn lookup(&self, key: Key) -> Data {
        T::lookup(self, key)
    }
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

            fn intern($($intern_in)*) -> $intern_out {
                $($intern_body)*
            }

            fn lookup($($lookup_in)*) -> $lookup_out {
                $($lookup_body)*
            }
        }
    };

    (impl[$($params:tt)*] Interner<$key_ty:ty, $data_ty:ty> for $base_ty:ty {
        delegate($field:ident)
    }) => {
        impl<$($params)*> $crate::neo::Interner<$key_ty, $data_ty> for $base_ty {
            fn as_dyn(&self) -> &dyn $crate::neo::Interner<$key_ty, $data_ty> {
                self
            }

            fn intern(&self, data: $data_ty) -> $key_ty {
                self.$field.intern(data)
            }

            fn lookup(&self, data: $key_ty) -> $data_ty {
                self.$field.lookup(data)
            }
        }
    };
}

/// Create a struct that implements `Interner` for various types
#[macro_export]
macro_rules! interner_struct {
    ($v:vis struct $n:ident { $($map_name:ident: $map_key:ty => $map_value:ty,)* }) => {
        $v struct $n {
            $($map_name: $crate::neo::table::InternTable<$map_key, $map_value>,)*
        }

        $(
            impl Interner<$map_key, $map_value> for $n {
                fn as_dyn(&self) -> &dyn Interner<Key, Key> {
                    self
                }

                fn intern(&self, data: Key) -> Key {
                    self.$map_name.intern(data)
                }

                fn lookup(&self, key: Key) -> Key {
                    self.$map_name.lookup(key)
                }
            }

            impl<Key> Interner<Key, Key> for $n {
                fn as_dyn(&self) -> &dyn Interner<Key, Key> {
                    self
                }

                fn intern(&self, data: Key) -> Key {
                    data
                }

                fn lookup(&self, key: Key) -> Key {
                    key
                }
            }
        )*
    }
}
