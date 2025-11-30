//! Contains all basic MOQT Types defined in the the Draft

pub mod config;
pub mod error_code;
pub mod message;
mod misc;
mod parameter;
mod request_id;
mod track;

pub use {
    misc::{
        AliasType, ContentExists, EndOfTrack, FetchType, FilterType, Forward, GroupOrder,
        JoiningFetch, Location, ReasonPhrase, StandaloneFetch,
    },
    parameter::{
        ClientSetupParameter, ClientSetupParameters, Parameter, Parameters, ServerSetupParameter,
        ServerSetupParameters, Token,
    },
    request_id::RequestId,
    track::{Name, Namespace},
};

pub mod error {
    pub use super::{
        parameter::{ClientSetupParameterError, ParameterError, ServerSetupParameterError},
        request_id::RequestIdError,
    };
}
