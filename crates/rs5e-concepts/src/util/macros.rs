macro_rules! implement_from_str {
    ($enum_name:ident, [$( [$str:expr, $variant:ident] ),*]) => {
        impl std::str::FromStr for $enum_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($str => Ok($enum_name::$variant),)*
                    _ => Err(()),
                }
            }
        }
    };
}

pub(crate) use implement_from_str; 