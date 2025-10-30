use proc_macro_error2::abort;
use syn::{FieldsNamed, Ident, Type, spanned::Spanned};

use crate::utils::attributes::FieldAttributes;

pub struct NamedFields {
    pub names: Vec<Ident>,
    pub tys: Vec<Type>,
    pub attrs: Vec<FieldAttributes>,
}

impl From<&FieldsNamed> for NamedFields {
    fn from(value: &FieldsNamed) -> Self {
        let mut names = Vec::new();
        let mut tys = Vec::new();
        let mut attrs = Vec::new();

        for field in &value.named {
            if let Some(ident) = field.ident.clone() {
                names.push(ident);
            } else {
                abort!(field.span(), "missing field name")
            }
            tys.push(field.ty.clone());
            attrs.push(FieldAttributes::from_attrs(&field.attrs));
        }

        Self { names, tys, attrs }
    }
}
