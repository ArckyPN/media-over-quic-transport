use crate::macro_helper::number_struct;

number_struct! {
    /// ## Unsubscribe
    ///
    /// Stops an active [Subscribe](crate::types::message::Subscribe).
    #[varint::draft_ref(v = 14)]
    Unsubscribe
    /// ## Request ID
    request_id
}
