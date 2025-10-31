use crate::macro_helper::number_struct;

number_struct! {
    /// TODO docs
    #[varint::draft_ref(v = 14, append = "-2")]
    MaxRequestId
    /// TODO docs
    request_id
}
