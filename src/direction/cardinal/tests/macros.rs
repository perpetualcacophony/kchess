macro_rules! assert_perpendicular_to {
    ($($a:ident { $( $b:ident $bool:literal )* } )*) => {
        paste::paste! { $($(

            #[test]
            fn [<$a:lower _ $b:lower>]() {
                assert_eq!(
                    super::Cardinal::$a.perpendicular_to(super::Cardinal::$b),
                    $bool
                )
            }

        )*)* }
    };
}
