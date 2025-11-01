use crate::macro_helper::number_struct;

number_struct! {
    /// Signals the Publisher that a Subscriber
    /// is no longer interested in receiving
    /// objects.
    FetchCancel
    /// The Request ID associated with the Fetch
    /// that is to be canceled.
    request_id
}
