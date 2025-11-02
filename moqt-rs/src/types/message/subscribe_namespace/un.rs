use crate::macro_helper::namespace_struct;

namespace_struct!(
    /// ## UnsubscribeNamespace
    ///
    /// Stops an active [SubscribeNamespace](crate::types::message::SubscribeNamespace).
    #[varint::draft_ref(v = 14)]
    UnsubscribeNamespace
    /// ## Track Namespace Prefix
    ///
    /// The Namespace Prefix to unsubscribe from.
    namespace_prefix
);
