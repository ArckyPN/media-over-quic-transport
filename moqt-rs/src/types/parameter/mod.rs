use {indexmap::IndexMap, varint::x};

mod client_setup_parameter;
mod server_setup_parameter;
mod token;
mod version_specific_parameter;

pub use {
    client_setup_parameter::{ClientSetupParameter, ClientSetupParameterError},
    server_setup_parameter::{ServerSetupParameter, ServerSetupParameterError},
    token::Token,
    version_specific_parameter::{Parameter, ParameterError},
};

pub type Parameters = IndexMap<x!(i), Parameter>;
pub type ClientSetupParameters = IndexMap<x!(i), ClientSetupParameter>;
pub type ServerSetupParameters = IndexMap<x!(i), ServerSetupParameter>;

#[cfg(test)]
mod tests {
    use {
        super::*,
        indexmap::IndexMap,
        pretty_assertions::assert_eq,
        std::time::Duration,
        varint::{
            VarInt,
            core::{ReferenceReader, ReferenceWriter, Writer},
            x,
        },
    };

    const PARAM_BUF: &[u8] = &[
        7,   // num parameters
        0x0, // generic number
        5,   //
        0x1, // generic bytes
        4,   // num bytes
        1, 2, 3, 4,   //
        0x2, // delivery timeout
        50,  //
        0x3, // auth token
        2,   // num bytes
        0,   // delete type
        6,   // alias
        0x4, // max cache
        34,  //
        0x5, // generic bytes
        4,   // num bytes
        10, 11, 12, 13,  //
        0x6, // generic number
        0,   //
    ];

    #[test]
    fn param_varint_test() {
        let mut reader = ReferenceReader::new(PARAM_BUF);

        let valid = IndexMap::<x!(i), Parameter>::decode(&mut reader, None);
        assert_eq!(
            valid,
            Ok((
                Parameters::from([
                    (0u8.into(), Parameter::Number(5u8.into())),
                    (1u8.into(), Parameter::Bytes([1, 2, 3, 4].into())),
                    (
                        2u8.into(),
                        Parameter::DeliveryTimeout(Duration::from_millis(50))
                    ),
                    (
                        3u8.into(),
                        Parameter::AuthorizationToken(Token::builder().delete().alias(6u8).build())
                    ),
                    (
                        4u8.into(),
                        Parameter::MaxCacheDuration(Duration::from_millis(34))
                    ),
                    (5u8.into(), Parameter::Bytes([10, 11, 12, 13].into())),
                    (6u8.into(), Parameter::Number(0u8.into()))
                ]),
                PARAM_BUF.len() * 8
            ))
        );

        let map = valid.expect("is ok").0;
        let mut writer = ReferenceWriter::new();
        let valid = map.encode(&mut writer, None);
        assert_eq!(valid, Ok(PARAM_BUF.len() * 8));
        assert_eq!(writer.finish(), Ok(PARAM_BUF.to_vec().into()));

        assert_eq!(map.len_bits(), Ok(PARAM_BUF.len() * 8));
    }

    // TODO same for client and server setup params
}
