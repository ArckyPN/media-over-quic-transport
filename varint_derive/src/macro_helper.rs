mod sub {
    macro_rules! ty_vec {
        (
            $($ty:ty),* $(,)?
        ) => {
            vec![
                $(
                    quote::quote! { $ty }
                ),*
            ]
        };
    }

    pub(crate) use ty_vec;
}

pub(crate) use sub::ty_vec;
