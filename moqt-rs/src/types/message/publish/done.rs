use crate::macro_helper::control_message_error;

control_message_error! {
    /// ## PublishDone
    ///
    /// Signals a Relay that a Publisher finished
    /// publishing Objects.
    #[derive(Debug, PartialEq, Clone)]
    #[varint::draft_ref(v = 14)]
    Publish + Done
}

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Publish + Done; NotSupported = 0x3);
}
