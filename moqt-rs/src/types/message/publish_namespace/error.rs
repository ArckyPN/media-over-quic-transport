use crate::macro_helper::control_message_error;

control_message_error!(
    /// ## PublishNamespaceError
    ///
    /// Response to a failed [PublishNamespace](crate::types::message::PublishNamespace).
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    PublishNamespace + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(PublishNamespace + Error; InternalError = 0);
}
