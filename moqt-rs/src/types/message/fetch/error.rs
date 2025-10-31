use crate::macro_helper::control_message_error;

control_message_error!(
    /// TODO docs
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    Fetch + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Fetch + Error; NotSupported = 0x3);
}
