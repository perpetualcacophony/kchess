macro_rules! bundle {
    ($ident:ident: $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $ident<'c> {
            $(
                pub $field: &'c $type
            ),*
        }

        impl<'c> $ident<'c> {
            pub fn get(components: &'c $crate::Components) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_ref()?
                    ),*
                })
            }
        }

        impl<'c> AsRef<Self> for $ident<'c> {
            fn as_ref(&self) -> &Self {
                self
            }
        }
    };

    (mut $ident:ident: $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $ident<'c> {
            $(
                pub $field: &'c mut $type
            ),*
        }

        impl<'c> $ident<'c> {
            pub fn get(components: &'c mut $crate::Components) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_mut()?
                    ),*
                })
            }
        }

        impl<'c> AsRef<Self> for $ident<'c> {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl<'c> AsMut<Self> for $ident<'c> {
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }
    };
}
