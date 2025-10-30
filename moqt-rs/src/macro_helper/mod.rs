mod control_message_error;
mod namespace_struct;
mod number_struct;

pub(crate) use control_message_error::control_message_error;
pub(crate) use namespace_struct::namespace_struct;
pub(crate) use number_struct::number_struct;

mod sub {
    // TODO come back to this later, might have to do with a proc macro attr
    macro_rules! draft_ref {
        (
            $draft:ident + $frag:literal
        ) => {
            paste::paste! {
                #[doc = "https://www.ietf.org/archive/id/draft-ietf-moq-transport-" $draft ".html#" $frag]
            }
        };
    }
    pub(crate) use draft_ref;
}

pub(crate) use sub::draft_ref;
