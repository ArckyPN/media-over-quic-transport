use crate::macro_helper::namespace_struct;

namespace_struct!(
    /// ## PublishNamespaceDone
    ///
    /// Stops an active [PublishNamespace](crate::types::message::PublishNamespace).
    #[varint::draft_ref(v = 14)]
    PublishNamespaceDone
    /// ## Track Namespace
    ///
    /// The affected Namespace.
    namespace
);
