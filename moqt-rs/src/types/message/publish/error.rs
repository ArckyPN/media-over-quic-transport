use crate::macro_helper::control_message_error;

control_message_error!(
    /// ## PublishError
    ///
    /// Response to a rejected [Publish](crate::types::message::Publish).
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    Publish + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Publish + Error; Uninterested = 0x4);
}
