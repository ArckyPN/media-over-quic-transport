mod sub {
    /// # Error Message Struct Constructor
    ///
    /// Build a public struct which has the
    /// typical Control Message Error shape:
    ///
    /// ```rust, ignore
    /// struct MessageError {
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
    ///     /// The Request ID associated with this Message.
    ///     pub request_id: varint::x!(i),
    ///     /// The Status Code.
    ///     pub code: error_code::StructIdent,
    ///     /// The Status Message.
    ///     ///
    ///     /// [ReasonPhrase]($crate::types::reason_phrase::ReasonPhrase)
    ///     pub reason: ReasonPhrase,
    /// }
    /// ```
    macro_rules! control_message_error {
        (
            $(#[$attrss:meta])*
            $name:ident + $ty:ident
        ) => {
            paste::paste! {
                #[derive(varint::VarInt, bon::Builder)]
                $(#[$attrss])*
                pub struct [< $name $ty >] {
                    /// ## Request ID
                    #[builder(into, setters(
                        name = id,
                        doc {
                            #[doc = "Sets the request ID on [" [< $name $ty >] "]."]
                        }
                    ))]
                    pub request_id: varint::x!(i),

                    /// ## Status Code
                    #[builder(into, setters(
                        doc {
                            #[doc = "Sets the status code on [" [< $name $ty >] "]."]
                        }
                    ))]
                    pub code: $crate::types::error_code::$name,

                    /// ## Status Message
                    ///
                    /// [ReasonPhrase]($crate::types::reason_phrase::ReasonPhrase)
                    #[builder(into, setters(
                        doc {
                            #[doc = "Sets the reason phrase on [" [< $name $ty >] "]."]
                        }
                    ))]
                    pub reason: $crate::types::misc::ReasonPhrase,
                }

                impl [< $name $ty >] {
                    /// Creates a new Instance.
                    pub fn new<ID, C, R>(id: ID, code: C, reason: R) -> Self
                    where
                        ID: Into<varint::x!(i)>,
                        C: Into<$crate::types::error_code::$name>,
                        R: Into<$crate::types::misc::ReasonPhrase>,
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
