pub trait TestData
where
    Self: std::marker::Sized,
{
    /// Returns test data
    fn test_data() -> Vec<(Self, Vec<u8>, usize)>;
}

mod sub {
    /// Used in unit testing to easily test all VarInt
    /// decoding and encoding.
    ///
    /// # Signature:
    ///
    /// ```ignore
    /// varint_enum_test!(
    ///     `Ident` of enum;
    ///     `Ident` of data buffer `&[u8]`;
    ///     `Literal` of invalid value `u8`;
    ///     `Ident`s of enum Variants, ...
    /// )
    /// ```
    macro_rules! varint_enum_test {
        (
            $name:ident;
            $buf:ident;
            $invalid:literal;
            $(
                $variants:ident
            ),+ $(,)?
        ) => {
            #[test]
            fn varint_test() {
                use varint::{Writer, VarInt};

            let mut reader = varint::core::ReferenceReader::new(&[$buf, &[$invalid]].concat());
            let mut writer = varint::core::ReferenceWriter::new();

            $({
                let valid = $name::decode(&mut reader, None);
                assert_eq!(valid, Ok(($name::$variants, 8)));

                let valid = $name::$variants.encode(&mut writer, None);
                assert_eq!(valid, Ok(8));
            })+

            let valid = writer.finish();
            assert_eq!(valid, Ok($buf.to_vec().into()));

            let invalid = $name::decode(&mut reader, None);
            assert_eq!(
                invalid,
                Err(varint::Error::UnknownValue { value: 0x3F })
            );
            }
        };
    }

    macro_rules! varint_struct_test {
        (
            $name:ident
        ) => {
            #[test]
            fn varint_test() {
                use varint::{VarInt, Writer};

                for (msg, buf, length) in $name::test_data() {
                    let mut reader = varint::core::ReferenceReader::new(&buf);
                    let valid = $name::decode(&mut reader, Some(length));
                    assert_eq!(valid, Ok((msg.clone(), length)));

                    let mut writer = varint::core::ReferenceWriter::new();
                    let valid = msg.encode(&mut writer, Some(length));
                    assert_eq!(valid, Ok(length));
                    assert_eq!(writer.finish(), Ok(buf.into()));
                }
            }
        };
    }

    macro_rules! control_message_error_test {
        (
            $name:ident + $ty:ident; $error:ident = $value:literal
        ) => {
            paste::paste! {
                use crate::test_helper::TestData;

                impl TestData for [< $name $ty >] {
                    fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
                        let v1 = Self::new(
                            13u8,
                            $crate::types::error_code::$name::$error,
                            "automatic error message"
                        );
                        let b1 = [
                            [
                                13, // ID
                                $value, // error code
                                23, // phrase length
                            ].to_vec(),
                            b"automatic error message".to_vec(),
                        ].concat();
                        let l1 = b1.len() * 8;

                        vec![(v1, b1, l1)]
                    }
                }

                $crate::test_helper::varint_struct_test!([< $name $ty >]);
            }
        };
    }

    pub(crate) use control_message_error_test;
    pub(crate) use varint_enum_test;
    pub(crate) use varint_struct_test;
}

pub(crate) use sub::*;
