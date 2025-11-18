use varint::varint_enum;

varint_enum! {
    /// ## End of Track
    ///
    /// Indicates whether there are still
    /// Objects to be published on a Track.
    ///
    /// [FetchOk](crate::types::message::FetchOk)
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(8))]
    pub enum EndOfTrack {
        /// ## True
        ///
        /// There will be more Objects to come.
        True = 0x1,

        /// ## False
        ///
        /// No more Object will be published.
        False = 0x0,
    }
}

impl From<bool> for EndOfTrack {
    fn from(value: bool) -> Self {
        if value {
            EndOfTrack::True
        } else {
            EndOfTrack::False
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x1, 0x0];

    varint_enum_test!(EndOfTrack; BUF; 0x3F;
        True, False,
    );
}
