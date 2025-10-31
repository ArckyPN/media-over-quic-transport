use crate::macro_helper::number_struct;

number_struct! {
    /// TODO docs
    #[varint::draft_ref(v = 14)]
    RequestsBlocked
    /// The Maximum Request ID for the session
    /// on which the endpoint is blocked.
    max_id
}
