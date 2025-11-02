use crate::macro_helper::control_message_error;

control_message_error!(
    /// ## SubscribeError
    ///
    /// Response to a failed [Subscribe](crate::type::message::Subscribe).
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    Subscribe + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Subscribe + Error; TrackDoesNotExist = 0x4);
}
