use varint::varint_enum;

varint_enum! {
    /// Describes whether or not an Object has
    /// been published on a Track.
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, rename = "section-9.8-3.5.1")]
    pub enum ContentExists {
        /// An Object has been published on
        /// this Track.
        Yes = 0x1,
        /// No Object has been published on
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
