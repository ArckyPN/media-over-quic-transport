mod sub {
    macro_rules! impl_from_msg_error {
        (
            $name:ident = [
                $($variants:ident => $tys:path),* $(,)?
            ]
        ) => {
            $(
                impl From<$tys> for $name {
                    fn from(value: $tys) -> Self {
                        Self::$variants {
                            msg: value.to_string(),
                        }
                    }
                }
            )*
        };
    }

    pub(crate) use impl_from_msg_error;
}

pub(crate) use sub::impl_from_msg_error;
