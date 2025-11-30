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

pub use {
    fetch::{Fetch, FetchCancel, FetchError, FetchOk},
    go_away::Goaway,
    max_request_id::MaxRequestId,
    publish::{Publish, PublishDone, PublishError, PublishOk},
    publish_namespace::{
        PublishNamespace, PublishNamespaceCancel, PublishNamespaceDone, PublishNamespaceError,
        PublishNamespaceOk,
    },
    requests_blocked::RequestsBlocked,
    setup::{ClientSetup, ServerSetup},
    subscribe::{Subscribe, SubscribeError, SubscribeOk, SubscribeUpdate, Unsubscribe},
    subscribe_namespace::{
        SubscribeNamespace, SubscribeNamespaceError, SubscribeNamespaceOk, UnsubscribeNamespace,
    },
    track_status::{TrackStatus, TrackStatusError, TrackStatusOk},
};

/// Message Configs
///
/// This module contains custom Configs with Builders for
/// more control over ControlMessages.
pub mod config {
    // TODO export all configs here
    // pub use super::subscribe::SubscribeConfig;
}

varint::varint_enum! {
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
    use std::time::Duration;

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

            let v2 = Self::ClientSetup(ClientSetup::builder().versions([1u8, 2u8, 3u8]).build());
            let b2 = vec![
                0x20, // Client Setup
                0,    // payload length
                5,    //
                3,    // num versions,
                1, 2, 3, // version
                0, // no parameters
            ];
            let l2 = b2.len() * 8;

            let v3 = Self::SubscribeOk(
                SubscribeOk::builder()
                    .id(9u8)
                    .alias(13u8)
                    .expires(Duration::from_millis(10))
                    .group_order(GroupOrder::Original)
                    .build(),
            );
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
