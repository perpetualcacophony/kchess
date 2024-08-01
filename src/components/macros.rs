macro_rules! bundle {
    ($ident:ident: $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $ident<'c, P = crate::pieces::Standard> {
            $(
                pub $field: &'c $type
            ),*
        }

        impl<'c, P> $ident<'c, P> {
            pub fn get(components: &'c $crate::Components<P>) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_ref()?
                    ),*
                })
            }
        }

        impl<'c, P> AsRef<Self> for $ident<'c, P> {
            fn as_ref(&self) -> &Self {
                self
            }
        }
    };

    (mut $ident:ident: $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $ident<'c, P> {
            $(
                pub $field: &'c mut $type
            ),*
        }

        impl<'c, P> $ident<'c, P> {
            pub fn get(components: &'c mut $crate::Components<P>) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_mut()?
                    ),*
                })
            }
        }

        impl<'c, P> AsRef<Self> for $ident<'c, P> {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl<'c, P> AsMut<Self> for $ident<'c, P> {
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }
    };
}
