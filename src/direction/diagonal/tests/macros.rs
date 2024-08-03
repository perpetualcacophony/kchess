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

macro_rules! parse_step {
    ($( $ranks:literal, $files:literal $diagonal:ident )*) => {paste::paste!{
        mod parse_step {
            use crate::{Direction, direction::Step};

            $(
                #[test]
                fn [<step_ $ranks _ $files _ $diagonal:lower>]() {
                    assert_eq!(
                        super::Diagonal::parse_step(Step::new($ranks, $files)),
                        Some(super::Diagonal::$diagonal)
                    )
                }
            )*
        }
    }};
}
