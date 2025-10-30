use varint::varint_enum;

varint_enum! {
    /// An Integer defining the serialization and
    /// processing behavior of a [Token](crate::types::Token).
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum AliasType {
        /// There is an Alias but no Type or Value.
        ///
        /// This Alias and the Token Value it was
        /// previously associated with MUST be
        /// retired.
        ///
        /// Retiring removes them from the pool
        /// of actively registered tokens.
        Delete  = 0x0,
        /// There is an Alias, a Type and a Value.
        ///
        /// This Alias MUST be associated with the
        /// Token Value for the duration of the
        /// Session or it is deleted.
        ///
        /// This action is termed "registering"
        /// the Token.
        Register = 0x1,
        /// There is an Alias but no Type or Value.
        ///
        /// Use the Token Type and Value previously
        /// registered with this Alias.
        UseAlias = 0x2,
        /// There is no Alias and there is a Type
        /// and Value.
        ///
        /// Use the Token Value as provided.
        ///
        /// The Token Value may be discarded after
        /// processing.
        UseValue = 0x3,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::varint_enum_test;

    use super::*;

    const BUF: &[u8] = &[0, 1, 2, 3];

    varint_enum_test!(AliasType; BUF; 0x3F;
        Delete, Register,
        UseAlias, UseValue,
    );
}
