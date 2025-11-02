use crate::macro_helper::number_struct;

number_struct! {
    /// ## PublishNamespaceOk
    ///
    /// Response to a successful [PublishNamespace](crate::types::message::PublishNamespace).
    #[varint::draft_ref(v = 14)]
    PublishNamespaceOk
    /// ## Request ID
    request_id
}
