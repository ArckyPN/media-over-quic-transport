mod sub {
    /// # Error Message Struct Constructor
    ///
    /// Build a public struct which has the
    /// typical Control Message Error shape:
    ///
    /// ```rust
    /// struct {
    ///     request_id: varint::x!(i),
    ///     code: ErrorCodeEnum,
    ///     reason: ReasonPhrase,
    /// }
    /// ```
    ///
    /// This also generates a bunch of impl block
    /// associated to these types.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// control_message_error! {
    ///     /// all attributes are passed through to the struct
    ///     StructIdent + CodeType
    /// }
    /// ```
    ///
    /// ## Generated Code
    ///
    /// ```rust,ignore
    /// /// all attributes are passed through to the struct
    /// pub struct StructIdent {
    ///     /// The Request ID associated with this Error
    ///     request_id: varint::x!(i),
    ///     /// The Status Code
    ///     code: error_code::StructIdent,
    ///     /// Status Message
    ///     reason: ReasonPhrase,
    /// }
    /// ```
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
                    /// The Status Code
                    code: $crate::types::error_code::$name,
                    /// Status Message
                    reason: $crate::types::reason_phrase::ReasonPhrase,
                }

                impl [< $name $ty >] {
                    /// Creates a new Instance.
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
                }
            }
        };
    }
    pub(crate) use control_message_error;
}

pub(crate) use sub::control_message_error;
