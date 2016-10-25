macro_rules! string_enum {
    ($ty:ident: $($key:ident => $value:expr,)*) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $ty {
            $($key),*
        }

        impl $ty {
            fn as_str(&self) -> &'static str {
                match *self {
                    $($ty::$key => $value),*
                }
            }

            fn try_from(str: &str) -> Option<Self> {
                match str {
                    $($value => Some($ty::$key),)*
                    _ => None,
                }
            }
        }

        impl ::std::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(self.as_str(), fmt)
            }
        }
    };
}
