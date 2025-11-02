use crate::macro_helper::number_struct;

number_struct! {
    /// ## SubscribeNamespaceOk
    ///
    /// Response to a successful [SubscribeNamespace](crate::types::message::SubscribeNamespace).
    #[varint::draft_ref(v = 14)]
    SubscribeNamespaceOk
    /// ## Request ID
    request_id
}
