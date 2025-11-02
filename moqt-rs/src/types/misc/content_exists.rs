use varint::varint_enum;

varint_enum! {
    /// ## Content Exists
    ///
    /// Describes whether or not an Object has
    /// been published on a Track.
    ///
    /// [Publish](crate::types::message::Publish)
    /// [SubscribeOk](crate::types::message::SubscribeOk)
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, rename = "section-9.8-3.5.1")]
    pub enum ContentExists {
        /// ## Yes
        ///
        /// An Object has been published on
        /// this Track.
        Yes = 0x1,

        /// No
        ///
        /// No Objects has been published on
        /// this Track.
        No = 0x0,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0x1, 0x0];

    varint_enum_test!(ContentExists; BUF; 0x3F;
        Yes, No,
    );
}
