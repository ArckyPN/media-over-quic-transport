use varint::varint_enum;

varint_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[varint(value = x(i))]
    pub enum ContentExists {
        Yes = 0x1,
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
