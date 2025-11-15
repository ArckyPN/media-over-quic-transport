use syn::Fields;

use super::{NamedFields, UnnamedFields};

#[derive(Default)]
pub enum EnumField {
    Struct(NamedFields),
    Tuples(UnnamedFields),
    #[default]
    Unit,
}

impl From<&Fields> for EnumField {
    fn from(value: &Fields) -> Self {
        match value {
            Fields::Unit => Self::Unit,
            Fields::Named(field) => Self::Struct(NamedFields::from(field)),
            Fields::Unnamed(field) => Self::Tuples(UnnamedFields::from(field)),
        }
    }
}
