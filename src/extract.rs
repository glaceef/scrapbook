pub trait FromMasterData<'a> {
    type MasterData;

    fn from_master_data(master_data: &'a Self::MasterData) -> Self;
}

#[doc(hidden)]
mod impl_for_tuples {
    use super::*;
    use paste::paste;
    use seq_macro::seq;

    macro_rules! impls {
        ( $($t:ident,)* ) => {
            impl<'a, __D $(, $t)*> FromMasterData<'a> for ($($t,)*)
            where
                $(
                    $t: FromMasterData<'a, MasterData = __D>,
                )*
            {
                type MasterData = __D;

                fn from_master_data<'b>(master_data: &'a Self::MasterData) -> Self {
                    (
                        $(
                            $t::from_master_data(master_data),
                        )*
                    )
                }
            }
        }
    }

    macro_rules! impls_tuple {
        ($n:expr) => {
            seq!(N in 1..=$n {
                paste! {
                    impls!( #( [<T~N>], )* );
                }
            });
        };
    }

    macro_rules! impls_tuple_for {
        ($n:expr) => {
            seq!(N in 1..=$n {
                impls_tuple!(N);
            });
        }
    }

    // Implementation for tuples with up to 12 items long.
    // See https://qiita.com/9laceef/items/e24f9541ef3924112f6b for these macros.
    impls_tuple_for!(12);
}
