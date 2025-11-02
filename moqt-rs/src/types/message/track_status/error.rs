use crate::macro_helper::control_message_error;

control_message_error!(
    /// ## TrackStatusError
    ///
    /// Response to a failed [TrackStatus](crate::types::message::TrackStatus)
    /// Message.
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    TrackStatus + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(TrackStatus + Error; ExpiredAuthToken = 0x12);
}
