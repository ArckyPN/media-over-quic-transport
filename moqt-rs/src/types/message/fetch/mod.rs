mod cancel;
mod error;
mod ok;

pub use {cancel::FetchCancel, error::FetchError, ok::FetchOk};

use {
    crate::types::{
        Parameters,
        misc::{FetchType, GroupOrder, JoiningFetch, StandaloneFetch},
    },
    bon::bon,
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
// use fetch_with_relative_joining_builder
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

use fetch_builder::{IsUnset, SetFetchType, SetJoining, SetStandalone, State};
impl<S: State> FetchBuilder<S>
where
    S::FetchType: IsUnset,
    S::Standalone: IsUnset,
    S::Joining: IsUnset,
{
    /// Sets this [Fetch] to a [StandaloneFetch].
    pub fn standalone_fetch(
        self,
        standalone: StandaloneFetch,
    ) -> FetchBuilder<SetFetchType<SetStandalone<SetJoining<S>>>> {
        let this = self.joining_internal(None);
        let this = this.standalone_internal(Some(standalone));
        this.fetch_type_internal(FetchType::Standalone)
    }

    /// Sets this [Fetch] to a relative [JoiningFetch].
    pub fn relative_joining_fetch(
        self,
        joining: JoiningFetch,
    ) -> FetchBuilder<SetFetchType<SetJoining<SetStandalone<S>>>> {
        let this = self.standalone_internal(None);
        let this = this.joining_internal(Some(joining));
        this.fetch_type_internal(FetchType::RelativeJoining)
    }

    /// Sets this [Fetch] to an absolute [JoiningFetch].
    pub fn absolute_joining_fetch(
        self,
        joining: JoiningFetch,
    ) -> FetchBuilder<SetFetchType<SetJoining<SetStandalone<S>>>> {
        let this = self.standalone_internal(None);
        let this = this.joining_internal(Some(joining));
        this.fetch_type_internal(FetchType::AbsoluteJoining)
    }
}

#[bon]
impl Fetch {
    #[builder(finish_fn = build)]
    pub fn new(
        #[builder(field)] parameters: Parameters,

        #[builder(into, setters(
            name = id,
            doc {
                /// Sets the request ID on [Fetch].
            }
        ))]
        request_id: x!(i),

        #[builder(
            name = sub_prio,
            with = |p: u8| <x!(8)>::try_from(p).expect("u8 will fit into 8 bits"), 
            setters(
                doc {
                    /// Sets the subscriber priority on [Fetch].
                }
        ))]
        subscriber_priority: x!(8),

        #[builder(setters(
            doc {
                /// Sets the group order on [Fetch].
            }
        ))]
        group_order: GroupOrder,

        #[builder(setters(vis = "", name = fetch_type_internal))] fetch_type: FetchType,

        #[builder(setters(vis = "", name = standalone_internal))] standalone: x!([StandaloneFetch]),

        #[builder(setters(vis = "", name = joining_internal))] joining: x!([JoiningFetch]),
    ) -> Self {
        Self {
            request_id,
            subscriber_priority,
            group_order,
            fetch_type,
            standalone,
            joining,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        test_helper::{TestData, varint_struct_test},
        types::parameter::Token,
    };

    use super::*;

    impl TestData for Fetch {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder()
                .id(0u16)
                .sub_prio(64)
                .group_order(GroupOrder::Descending)
                .standalone_fetch(
                    StandaloneFetch::builder()
                        .namespace(["moqt"])
                        .name("vod")
                        .start(0u8, 0u8)
                        .end(15u8, 15u8)
                        .build(),
                )
                .build();
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

            let v2 = Self::builder()
                .id(9u8)
                .sub_prio(13)
                .group_order(GroupOrder::Ascending)
                .relative_joining_fetch(JoiningFetch::builder().id(10u8).start(5u8).build())
                .auth_token(Token::builder().delete().alias(7u8).build())
                .build();
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

            let v3 = Self::builder()
                .id(33u8)
                .sub_prio(0)
                .auth_token(Token::builder().delete().alias(7u8).build())
                .number(10u8, 21u8)
                .group_order(GroupOrder::Original)
                .absolute_joining_fetch(JoiningFetch::builder().id(44u8).start(1u16).build())
                .build();
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
