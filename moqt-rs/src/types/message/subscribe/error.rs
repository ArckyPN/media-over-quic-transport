use crate::macro_helper::control_message_error;

control_message_error!(
    /// # TODO
    #[derive(Debug, PartialEq, Clone)]
    Subscribe + Error
);

#[cfg(test)]
mod tests {
    use crate::test_helper::control_message_error_test;

    use super::*;

    control_message_error_test!(Subscribe + Error; TrackDoesNotExist = 0x4);
}
