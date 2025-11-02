use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, DataEnum, Ident, Lit, Type, spanned::Spanned, token::Brace};

use crate::{
    crate_name,
    macro_helper::ty_vec,
    utils::{
        add_value_unique,
        attributes::{EnumAttributes, EnumVariantAttributes, FieldAttributes},
        fields::EnumField,
    },
};

pub struct ImplEnum {
    /// name of the enum being derived upon
    name: Ident,

    /// name of the crate
    varint: Ident,

    /// type of the variant value
    value_ty: Type,

    /// names of the enum Variants
    variants: Vec<Ident>,

    /// literal values of the variants
    ///
    /// parsed from the attribute `#[varint(value = <lit>)]`
    /// (the value given to the macro `<variant> = <lit>`)
    values: Vec<Lit>,

    /// optional fields of each variant
    fields: Vec<EnumField>,
}

impl ImplEnum {
    pub fn new(name: &Ident, data: &DataEnum, attrs: &[Attribute]) -> Self {
        let attrs = EnumAttributes::from_attrs(attrs);
        let value_ty = attrs.value;

        let mut variants = Vec::default();
        let mut values = Vec::default();
        let mut fields = Vec::default();
        for variant in &data.variants {
            variants.push(variant.ident.clone());

            let va = EnumVariantAttributes::from_attrs(&variant.attrs, variant.span());
            add_value_unique(va.value, &mut values);

            fields.push(EnumField::from(&variant.fields));
        }

        Self {
            name: name.clone(),
            varint: crate_name(),
            value_ty,
            variants,
            values,
            fields,
        }
    }

    fn decoders(&self) -> Vec<TokenStream> {
        let varint = &self.varint;

        self.fields
            .iter()
            .zip(&self.variants)
            .map(|(field, variant)| match field {
                EnumField::Unit => quote! { Ok((Self::#variant, bits)) },
                EnumField::Struct(s) => {
                    let names = &s.names;
                    let tys = &s.tys;
                    let length_handle = quote_field_attrs_decode(&s.attrs, &self.varint);

                    quote! {{
                        #(
                            #length_handle
                            let (#names, len) = <#tys as #varint::VarInt>::decode(reader, decode_len)?;
                            bits += len;
                        )*
                        Ok(
                            (
                                Self::#variant { #( #names ),* },
                                bits
                            )
                        )
                    }}
                }
                EnumField::Tuples(t) => {
                    let tys = &t.tys;
                    let length_handle = quote_field_attrs_decode(&t.attrs, &self.varint);
                    let names = tys.iter().enumerate().map(|(idx,_)| format_ident!("v{}", idx)).collect::<Vec<_>>();

                    quote! {{
                        #(
                            #length_handle
                            let (#names, len) = <#tys as #varint::VarInt>::decode(reader, decode_len)?;
                            bits += len;
                        )*
                        Ok(
                            (
                                Self::#variant( #( #names ),* ),
                                bits
                            )
                        )
                    }}
                }
            })
            .collect()
    }

    fn encoders(&self) -> Vec<TokenStream> {
        let value_ty = &self.value_ty;
        self.fields
            .iter()
            .zip(&self.variants)
            .zip(&self.values)
            .map(|((field, variant), value)| {
                let value_encoder = quote! {
                    let num = <#value_ty>::try_from(#value)?;
                    bits += num.encode(writer, None)?;
                };

                match field {
                    EnumField::Unit => quote! {
                        Self::#variant => {
                            #value_encoder
                        }
                    },
                    EnumField::Struct(s) => {
                        let names = &s.names;
                        let length_handle = quote_field_attrs_encode(&s.attrs, names);

                        quote! {
                            Self::#variant { #( #names ),* } => {
                                #value_encoder
                                #(
                                    #length_handle
                                    bits += #names.encode(writer, encode_length)?;
                                )*
                            }
                        }
                    }
                    EnumField::Tuples(t) => {
                        let tys = &t.tys;
                        let names = tys
                            .iter()
                            .enumerate()
                            .map(|(idx, _)| format_ident!("v{}", idx))
                            .collect::<Vec<_>>();
                        let length_handle = quote_field_attrs_encode(&t.attrs, &names);

                        quote! {
                            Self::#variant ( #( #names ),* ) => {
                                #value_encoder
                                #(
                                    #length_handle
                                    bits += #names.encode(writer, encode_length)?;
                                )*
                            }
                        }
                    }
                }
            })
            .collect()
    }

    fn partial_eq(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        let fields = &self.fields;
        let values = &self.values;
        let tys = ty_vec!(
            u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
        );

        let eq_val = tys
            .iter()
            .map(|ty| {
                let variant = variants
                    .iter()
                    .zip(fields)
                    .map(|(variant, field)| match field {
                        EnumField::Unit => {
                            quote! { #variant }
                        }
                        EnumField::Struct(s) => {
                            let names = &s.names;
                            quote! { #variant { #( #names ),* } }
                        }
                        EnumField::Tuples(t) => {
                            let names = t
                                .tys
                                .iter()
                                .enumerate()
                                .map(|(idx, _)| format_ident!("v{}", idx))
                                .collect::<Vec<_>>();
                            quote! { #variant ( #( #names ),* ) }
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    let val = match self {
                        #(
                            Self::#variant => #values as #ty
                        ),*
                    };
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(
                impl PartialEq<#tys> for #name {
                    fn eq(&self, other: &#tys) -> bool {
                        #eq_val
                        val == *other
                    }
                }
                impl PartialEq<#name> for #tys {
                    fn eq(&self, other: &#name) -> bool {
                        other == self
                    }
                }
            )*
        }
        .to_tokens(tokens);
    }

    fn len_bits(&self) -> Vec<TokenStream> {
        let value_ty = &self.value_ty;

        self.variants.iter().zip(&self.values).zip(&self.fields).map(|((variant, value), fields)| {
            let bit_len = quote! {
                <#value_ty>::try_from(#value).expect("# TODO: how do deal with this?").len_bits()
            };

            match fields {
                EnumField::Unit => {
                    quote! {
                        Self::#variant => #bit_len
                    }
                }
                EnumField::Struct(s) => {
                    let names = &s.names;
                    quote! {
                        Self::#variant { #(#names),* } => #bit_len
                    }
                }
                EnumField::Tuples(t) => {
                    let names = t.tys.iter().enumerate().map(|(idx,_)| format_ident!("v{}", idx)).collect::<Vec<_>>();
                    quote! {
                        Self::#variant ( #(#names),* ) => #bit_len
                    }
                }
            }
        }).collect()
    }

    fn impl_varint(&self, tokens: &mut proc_macro2::TokenStream) {
        let varint = &self.varint;
        let name = &self.name;
        let value_ty = &self.value_ty;
        let values = &self.values;

        let decoders = self.decoders();
        let encoders = self.encoders();

        let len_bits = self.len_bits();

        // TODO impl a function which return the value of an variant
        quote! {
            impl #varint::VarInt for #name
        }
        .to_tokens(tokens);
        Brace::default().surround(tokens, |tokens| {
            quote! {
                type Error = #varint::Error;

                fn decode<R>(reader: &mut R, length: Option<usize>) -> Result<(Self, usize), Self::Error>
                where
                    R: #varint::Reader,
                    Self: std::marker::Sized,
                {
                    use #varint::snafu::ResultExt;

                    let mut bits = 0;
                    let (num, len) = <#value_ty as #varint::VarInt>::decode(reader, None)?;
                    bits += len;

                    match num.number::<u128>() {
                        #(
                            #values => #decoders
                        ),*, // FIXME without these extra commas the parsing breaks
                        x => Err(Self::Error::UnknownValue { value: x }),
                    }
                }

                fn encode<W>(&self, writer: &mut W, length: Option<usize>) -> Result<usize, Self::Error>
                where
                    W: #varint::Writer,
                {
                    let mut bits = 0;
                    match self {
                        #(
                            #encoders
                        )*
                    }
                    Ok(bits)
                }

                fn len_bits(&self) -> usize {
                    match self {
                        #( #len_bits ),*
                    }
                }

                fn length_required() -> bool {
                    false // TODO properly
                }
            }.to_tokens(tokens);
        });
    }
}

impl ToTokens for ImplEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_varint(tokens);
        self.partial_eq(tokens);
    }
}

fn quote_field_attrs_decode(attrs: &[FieldAttributes], varint: &Ident) -> Vec<TokenStream> {
    attrs
        .iter()
        .map(|attr| {
            if let Some(length) = &attr.length {
                quote! {
                    let (decode_len, len) = <#length as #varint::VarInt>::decode(reader, None)?;
                    bits += len;
                    let decode_len = Some(decode_len.number::<usize>() * 8);
                }
            } else {
                quote! {
                    let decode_len = None;
                }
            }
        })
        .collect()
}

fn quote_field_attrs_encode(attrs: &[FieldAttributes], names: &[Ident]) -> Vec<TokenStream> {
    attrs
        .iter()
        .zip(names)
        .map(|(attr, name)| {
            if let Some(length) = &attr.length {
                quote! {
                    let len_bits = #name.len_bits();
                    let num = <#length>::try_from(len_bits / 8)?;
                    bits += num.encode(writer, None)?;
                    let encode_length = Some(len_bits);
                }
            } else {
                quote! {
                    let encode_length = None;
                }
            }
        })
        .collect()
}
