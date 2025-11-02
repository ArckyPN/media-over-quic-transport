//! # Control Messages
//!
//! This module contains all Control Messages.

mod fetch;
mod go_away;
mod max_request_id;
mod publish;
mod publish_namespace;
mod requests_blocked;
mod setup;
mod subscribe;
mod subscribe_namespace;
mod track_status;

pub use fetch::{Fetch, FetchCancel, FetchError, FetchOk};
pub use go_away::Goaway;
pub use max_request_id::MaxRequestId;
pub use publish::{Publish, PublishDone, PublishError, PublishOk};
pub use publish_namespace::{
    PublishNamespace, PublishNamespaceCancel, PublishNamespaceDone, PublishNamespaceError,
    PublishNamespaceOk,
};
pub use requests_blocked::RequestsBlocked;
pub use setup::{ClientSetup, ServerSetup};
pub use subscribe::{Subscribe, SubscribeError, SubscribeOk, SubscribeUpdate, Unsubscribe};
pub use subscribe_namespace::{
    SubscribeNamespace, SubscribeNamespaceError, SubscribeNamespaceOk, UnsubscribeNamespace,
};
pub use track_status::{TrackStatus, TrackStatusError, TrackStatusOk};

use varint::varint_enum;

varint_enum! {
    length {
        tuple[0] = x(16)
    }
    /// ## Control Messages
    ///
    /// All possible Control Messages.
    #[derive(Debug, PartialEq, Clone)]
    #[varint(value = x(i))]
    #[varint::draft_ref(v = 14, rename = "control-messages")]
    pub enum ControlMessage {
        ClientSetup(ClientSetup) = 0x20,
        ServerSetup(ServerSetup) = 0x21,
        GoAway(Goaway) = 0x10,
        MaxRequestId(MaxRequestId) = 0x15,
        RequestsBlocked(RequestsBlocked) = 0x1A,
        Subscribe(Subscribe) = 0x3,
        SubscribeOk(SubscribeOk) = 0x4,
        SubscribeError(SubscribeError) = 0x5,
        SubscribeUpdate(SubscribeUpdate) = 0x2,
        Unsubscribe(Unsubscribe) = 0xA,
        PublishDone(PublishDone) = 0xB,
        Publish(Publish) = 0x1D,
        PublishOk(PublishOk) = 0x1E,
        PublishError(PublishError) = 0x1F,
        Fetch(Fetch) = 0x16,
        FetchOk(FetchOk) = 0x18,
        FetchError(FetchError) = 0x19,
        FetchCancel(FetchCancel) = 0x17,
        TrackStatus(TrackStatus) = 0xD,
        TrackStatusOk(TrackStatusOk) = 0xE,
        TrackStatusError(TrackStatusError) = 0xF,
        PublishNamespace(PublishNamespace) = 0x6,
        PublishNamespaceOk(PublishNamespaceOk) = 0x7,
        PublishNamespaceError(PublishNamespaceError) = 0x8,
        PublishNamespaceDone(PublishNamespaceDone) = 0x9,
        PublishNamespaceCancel(PublishNamespaceCancel) = 0xC,
        SubscribeNamespace(SubscribeNamespace) = 0x11,
        SubscribeNamespaceOk(SubscribeNamespaceOk) = 0x12,
        SubscribeNamespaceError(SubscribeNamespaceError) = 0x13,
        UnsubscribeNamespace(UnsubscribeNamespace) = 0x14,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_helper::{TestData, varint_struct_test},
        types::{error_code, misc::GroupOrder},
    };

    use super::*;

    impl TestData for ControlMessage {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::FetchError(FetchError::new(
                50u8,
                error_code::Fetch::InvalidRange,
                "error",
            ));
            let b1 = [
                vec![
                    0x19, // Fetch Error
                    0,    // payload length
                    8,    // (16 bits)
                    50,   // id
                    5,    // invalid range code
                    5,    // reason length
                ],
                b"error".to_vec(), // reason
            ]
            .concat();
            let l1 = b1.len() * 8;

            let v2 = Self::ClientSetup(ClientSetup::new(&[1u8, 2u8, 3u8]));
            let b2 = vec![
                0x20, // Client Setup
                0,    // payload length
                4,    //
                3,    // num versions,
                1, 2, 3, // version
                   // TODO parameters
            ];
            let l2 = b2.len() * 8;

            let v3 = Self::SubscribeOk(SubscribeOk::new_no_content(
                9u8,
                13u8,
                10u8,
                GroupOrder::Original,
            ));
            let b3 = vec![
                0x4, // Subscribe Ok
                0,   // payload length
                6,   //
                9,   // ID 9
                13,  // track alias 13
                10,  // expires in 10ms
                0,   // original group order
                0,   // content doesn't exist
                // largest loc not needed
                0, // no parameters
            ];
            let l3 = b3.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3)]
        }
    }

    varint_struct_test!(ControlMessage);
}
