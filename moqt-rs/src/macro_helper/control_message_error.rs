mod sub {
    macro_rules! control_message_error {
        (
            $(#[$attrss:meta])*
            $name:ident + $ty:ident
        ) => {
            paste::paste! {
                #[derive(varint::VarInt)]
                $(#[$attrss])*
                pub struct [< $name $ty >] {
                    /// The Request ID associated with this Error
                    request_id: varint::x!(i),
                    /// The Error Code
                    code: $crate::types::error_code::$name,
                    /// Error Message
                    reason: $crate::types::reason_phrase::ReasonPhrase,
                }

                impl [< $name $ty >] {
                    /// TODO docs
                    pub fn new<ID, C, R>(id: ID, code: C, reason: R) -> Self
                    where
                        ID: Into<varint::x!(i)>,
                        C: Into<$crate::types::error_code::$name>,
                        R: Into<$crate::types::reason_phrase::ReasonPhrase>,
                    {
                        Self {
                            request_id: id.into(),
                            code: code.into(),
                            reason: reason.into(),
                        }
                    }

                    // TODO impl all stuff here for usability
                }
            }
        };
    }
    pub(crate) use control_message_error;
}

pub(crate) use sub::control_message_error;
