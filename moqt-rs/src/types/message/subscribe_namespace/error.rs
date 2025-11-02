use crate::macro_helper::control_message_error;

control_message_error!(
    /// ## SubscribeNamespaceError
    ///
    /// Response to a failed [SubscribeNamespace](crate::types::message::SubscribeNamespace).
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    SubscribeNamespace + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(SubscribeNamespace + Error; NamespacePrefixOverlap = 0x5);
}
