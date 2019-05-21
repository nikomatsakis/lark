#[macro_export]
macro_rules! trait_alias {
    ($name:ident = $($bounds:tt)*) => {
        trait $name: $($bounds)* { }

        impl<T> $name for T
        where T: $($bounds)*
        {
        }
    }
}
