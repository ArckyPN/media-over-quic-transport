use syn::{FieldsUnnamed, Type};

use crate::utils::attributes::FieldAttributes;

pub struct UnnamedFields {
    pub tys: Vec<Type>,
    pub attrs: Vec<FieldAttributes>,
}

impl From<&FieldsUnnamed> for UnnamedFields {
    fn from(value: &FieldsUnnamed) -> Self {
        let mut tys = Vec::new();
        let mut attrs = Vec::new();

        for field in &value.unnamed {
            tys.push(field.ty.clone());
            attrs.push(FieldAttributes::from_attrs(&field.attrs));
        }

        Self { tys, attrs }
    }
}
