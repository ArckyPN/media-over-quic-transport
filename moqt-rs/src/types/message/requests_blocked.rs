use crate::macro_helper::number_struct;

number_struct! {
    /// ## RequestsBlocked
    ///
    /// The Request ID a new Request exceeds
    /// the maximum Request ID.
    #[varint::draft_ref(v = 14)]
    RequestsBlocked
    /// ## Maximum Request ID
    ///
    /// The Maximum Request ID for the session
    /// on which the endpoint is blocked.
    max_id
}
