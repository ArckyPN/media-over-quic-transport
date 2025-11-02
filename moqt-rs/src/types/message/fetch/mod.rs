mod cancel;
mod error;
mod ok;

pub use {cancel::FetchCancel, error::FetchError, ok::FetchOk};

use {
    crate::types::{
        Parameters,
        misc::{FetchType, GroupOrder, JoiningFetch, StandaloneFetch},
    },
    varint::{VarInt, x},
};

/// ## Fetch
///
/// Request Objects that have already been published.
///
/// There are three types of Fetches:
///
/// ### Standalone Fetch
///
/// Independently fetching objects from a Track with
/// a specified range of objects.
///
/// ### Relative Joining Fetch
///
/// Join an existing Subscription with a relative
/// starting point, i.e. the number of Groups before
/// the current one.
///
/// ### Absolute Joining Fetch
///
/// Join an existing Subscription from a specific
/// starting point, i.e. the specific Group to
/// start from.
///
/// ---
///
/// In both Joining Fetch cases the Publisher will
/// send past Object until the active Subscription
/// point has been reached. From there the Fetch
/// ends and the Subscribe takes over.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
#[varint(parameters(auth_token))]
pub struct Fetch {
    /// ## Request ID
    pub request_id: x!(i),

    /// ## Subscriber Priority
    ///
    /// Sets a priority in relation to all Fetches
    /// and Subscribes in the current Session.
    ///
    /// Lower means higher priority.
    pub subscriber_priority: x!(8),

    /// ## Group Order
    ///
    /// The order in which to receive Groups.
    ///
    /// [GroupOrder]
    pub group_order: GroupOrder,

    /// ## Fetch Type
    ///
    /// The type of Fetch.
    ///
    /// [FetchType]
    pub fetch_type: FetchType,

    /// ## Standalone Fetch
    ///
    /// Payload of a Standalone Fetch.
    ///
    /// Some when `fetch_type` is
    ///
    /// * [FetchType::Standalone]
    ///
    /// Otherwise None.
    ///
    /// [StandaloneFetch]
    #[varint(when(fetch_type = 0x1))]
    pub standalone: x!([StandaloneFetch]),

    /// ## Joining Fetch
    ///
    /// Payload of a Joining Fetch.
    ///
    /// Some when `fetch_type` is:
    ///
    /// * [FetchType::RelativeJoining]
    /// * [FetchType::AbsoluteJoining]
    ///
    /// Otherwise None.
    ///
    /// [JoiningFetch]
    #[varint(when(fetch_type = 0x2 || 0x3))]
    pub joining: x!([JoiningFetch]),

    /// ## Parameters
    ///
    /// [Parameters]
    pub parameters: Parameters,
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::{
        test_helper::{TestData, varint_struct_test},
        types::parameter::{Parameter, Token},
    };

    use super::*;

    impl TestData for Fetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Fetch {
                request_id: 0u16.into(),
                subscriber_priority: 64.try_into().unwrap(),
                group_order: GroupOrder::Descending,
                fetch_type: FetchType::Standalone,
                standalone: Some(StandaloneFetch::new(
                    ["moqt"],
                    "vod",
                    (0u8, 0u8),
                    [15u8, 15u8],
                )),
                joining: None,
                parameters: Parameters::default(),
            };
            let b1 = [
                [
                    0,  // ID
                    64, // sub prio
                    2,  // descending group order
                    1,  // standalone fetch type
                    1,  // num tuple elements
                    4,  // len first tuple
                ]
                .to_vec(),
                b"moqt".to_vec(),
                [
                    3, // len track name
                ]
                .to_vec(),
                b"vod".to_vec(),
                [
                    0,  // start group
                    0,  // start object
                    15, // end group
                    15, // end object
                    // joining not needed
                    0, // no parameters
                ]
                .to_vec(),
            ]
            .concat();
            let l1 = b1.len() * 8;

            let v2 = Fetch {
                request_id: 9u8.into(),
                subscriber_priority: <x!(8)>::new(13u8).unwrap(),
                group_order: GroupOrder::Ascending,
                fetch_type: FetchType::RelativeJoining,
                standalone: None,
                joining: Some(JoiningFetch::new(10u8, 5u8)),
                parameters: IndexMap::from([(
                    <x!(i)>::from(3u8),
                    Parameter::AuthorizationToken(Token::new_delete(7u8)),
                )])
                .into(),
            };
            let b2 = vec![
                9,  // ID 9
                13, // sub prio 9
                1,  // ascending group order
                2,  // relative joining fetch type
                // standalone not needed
                10, // joining fetch ID
                5,  // start number
                1,  // 1 parameter
                3,  // auth token type
                2,  // num bytes of token
                0,  // delete type
                7,  // alias
            ];
            let l2 = b2.len() * 8;

            let v3 = Self {
                request_id: 33u8.into(),
                subscriber_priority: 0.try_into().unwrap(),
                group_order: GroupOrder::Original,
                fetch_type: FetchType::AbsoluteJoining,
                standalone: None,
                joining: Some(JoiningFetch::new(44u8, 1u16)),
                parameters: IndexMap::from([
                    (
                        <x!(i)>::from(3u8),
                        Parameter::AuthorizationToken(Token::new_delete(7u8)),
                    ),
                    (<x!(i)>::from(10u8), Parameter::Number(21u8.into())),
                ])
                .into(),
            };
            let b3 = vec![
                33, 0, 0, 3, 44, 1,  // you get the point now
                2,  // 2 parameter
                3,  // first type
                2,  // num byte of token
                0,  // delete type
                7,  // alias
                10, // second type
                21, // number
            ];
            let l3 = b3.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3)]
        }
    }

    varint_struct_test!(Fetch);
}
