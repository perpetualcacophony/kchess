macro_rules! contains_cardinal {
    ( $( $diagonal:ident { $( $cardinal:ident $contains:literal )* } )* ) => {paste::paste!{
        mod contains_cardinal {
            use crate::{Direction, direction::Cardinal};

            $(
                $(
                    #[test]
                    fn [<$diagonal:lower _ $cardinal:lower>]() {
                        assert_eq!(
                            super::Diagonal::$diagonal.contains_cardinal(Cardinal::$cardinal),
                            $contains
                        )
                    }
                )*
            )*
        }
    }};
}

macro_rules! opposite {
    ( $( $diagonal:ident $opposite:ident )* ) => {paste::paste!{
        mod opposite {
            $(
                #[test]
                fn [<$diagonal:lower _ $opposite:lower>]() {
                    assert_eq!(
                        !super::Diagonal::$diagonal,
                        super::Diagonal::$opposite
                    )
                }
            )*
        }
    }};
}
