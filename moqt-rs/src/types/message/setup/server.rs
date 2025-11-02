use crate::macro_helper::number_struct;

number_struct! {
    /// ## ServerSetup
    ///
    /// The Response to the opening handshake
    /// Message [ClientSetup](crate::types::message::ClientSetup).
    #[varint::draft_ref(v = 14, rename = "client_setup-and-server_set")]
    ServerSetup
    /// ## Selected Version
    ///
    /// The selected Version chosen by the
    /// Server.
    selected_version
    // TODO parameters
}
