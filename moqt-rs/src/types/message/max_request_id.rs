use crate::macro_helper::number_struct;

number_struct! {
    /// ## MaxRequestId
    ///
    /// Sets a new limited on how many Requests
    /// are supported on the current Session.
    #[varint::draft_ref(v = 14, append = "-2")]
    MaxRequestId
    /// ## Maximum Request ID
    ///
    /// The maximum Request ID + 1.
    request_id
}
