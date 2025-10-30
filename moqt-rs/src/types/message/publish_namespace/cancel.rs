use crate::macro_helper::control_message_error;

control_message_error!(
    /// # TODO
    #[derive(Debug, PartialEq, Clone)]
    PublishNamespace + Cancel
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(PublishNamespace + Cancel; MalformedAuthToken = 0x10);
}
