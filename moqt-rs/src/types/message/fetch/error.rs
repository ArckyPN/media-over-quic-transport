use crate::macro_helper::control_message_error;

control_message_error!(
    /// # TODO
    #[derive(Debug, PartialEq, Clone)]
    Fetch + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Fetch + Error; NotSupported = 0x3);
}
