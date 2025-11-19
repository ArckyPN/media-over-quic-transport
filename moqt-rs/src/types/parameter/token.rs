use {
    crate::types::misc::AliasType,
    bon::Builder,
    snafu::{ResultExt, Snafu},
    std::fmt::Debug,
    varint::{
        VarInt, VarIntBytes, Writer,
        core::{ReferenceReader, ReferenceWriter, WriterError},
        x,
    },
};

/// ## Authorization Token
///
/// A Token delivered as [Parameter](crate::types::Parameters)
/// used to authenticate actors in the system,
/// whether or not they are allowed to perform
/// the actions they are trying to do.
#[derive(VarInt, PartialEq, Clone, Builder)]
pub struct Token {
    /// ## Token Type
    ///
    /// Defines how this Token is used.
    ///
    /// [AliasType]
    #[builder(setters(vis = "", name = alias_typ_internal))]
    pub alias_typ: AliasType,

    /// ## Token Alias
    ///
    /// An ID to identify and reference a
    /// Token Value.
    ///
    /// Some when `alias_type` is:
    ///
    /// * [Delete](AliasType::Delete)
    /// * [Register](AliasType::Register)
    /// * [UseAlias](AliasType::UseAlias)
    ///
    /// Otherwise None.
    #[builder(setters(vis = "", name = alias_internal))]
    #[varint(when(alias_typ = 0x0 || 0x1 || 0x2))]
    pub alias: x!([i]),

    /// ## Token Type
    ///
    /// Numeric ID of the type of Payload.
    ///
    /// Possible IDs are defined in [Draft](https://www.ietf.org/archive/id/draft-ietf-moq-transport-14.html#iana).
    ///
    /// 0 means that the type is signaled out-of-band.
    ///
    /// Some when `alias_type` is:
    ///
    /// * [Delete](AliasType::Delete)
    /// * [UseValue](AliasType::UseValue)
    ///
    /// Otherwise None.
    #[builder(setters(vis = "", name = typ_internal))]
    #[varint(when(alias_typ = 0x1 || 0x3))]
    pub typ: x!([i]), // TODO there are none defined right now. Once there this will likely be replaced by an Enum

    /// ## Token Value
    ///
    /// The actual Token payload, serialized
    /// as identified by `typ`.
    ///
    /// Some when `alias_type` is:
    ///
    /// * [Delete](AliasType::Delete)
    /// * [UseValue](AliasType::UseValue)
    ///
    /// Otherwise None.
    #[builder(setters(vis = "", name = value_internal))]
    #[varint(when(alias_typ = 0x1 || 0x3))]
    value: x!([..]),
}

use token_builder::{IsSet, IsUnset, SetAlias, SetAliasTyp, SetTyp, SetValue, State};
impl<S: State> TokenBuilder<S>
where
    S::AliasTyp: IsUnset,
    S::Alias: IsUnset,
    S::Typ: IsUnset,
    S::Value: IsUnset,
{
    /// Sets the type of this [Token] to [Delete](AliasType::Delete).
    pub fn delete(self) -> TokenBuilder<SetAliasTyp<SetValue<SetTyp<S>>>> {
        let this = self.typ_internal(None);
        let this = this.value_internal(None);
        this.alias_typ_internal(AliasType::Delete)
    }

    /// Sets the type of this [Token] to [Register](AliasType::Register).
    pub fn register(self) -> TokenBuilder<SetAliasTyp<S>> {
        self.alias_typ_internal(AliasType::Register)
    }

    /// Sets the type of this [Token] to [UseAlias](AliasType::UseAlias).
    pub fn use_alias(self) -> TokenBuilder<SetAliasTyp<SetValue<SetTyp<S>>>> {
        let this = self.typ_internal(None);
        let this = this.value_internal(None);
        this.alias_typ_internal(AliasType::UseAlias)
    }

    pub fn use_value(self) -> TokenBuilder<SetAliasTyp<SetAlias<S>>> {
        let this = self.alias_internal(None);
        this.alias_typ_internal(AliasType::UseValue)
    }
}

impl<S: State> TokenBuilder<S>
where
    S::AliasTyp: IsSet,
{
    /// Sets the alias of this [Token].
    pub fn alias<A>(self, alias: A) -> TokenBuilder<SetAlias<S>>
    where
        A: Into<x!(i)>,
        S::Alias: IsUnset,
    {
        self.alias_internal(Some(alias.into()))
    }

    /// Sets the type of this [Token].
    pub fn typ<T>(self, typ: T) -> TokenBuilder<SetTyp<S>>
    where
        T: Into<x!(i)>,
        S::Typ: IsUnset,
    {
        self.typ_internal(Some(typ.into()))
    }

    /// Sets the value of this [Token].
    pub fn value<V>(self, value: V) -> TokenBuilder<SetValue<S>>
    where
        V: Into<x!(..)>,
        S::Value: IsUnset,
    {
        self.value_internal(Some(value.into()))
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("alias_typ", &self.alias_typ)
            .field("alias", &self.alias.as_ref().map(|a| a.to_string()))
            .field("type", &self.typ.as_ref().map(|a| a.to_string()))
            .field("value", &self.value)
            .finish()
    }
}

impl TryFrom<x!(..)> for Token {
    type Error = TokenError;
    fn try_from(value: x!(..)) -> Result<Self, Self::Error> {
        let buf = value.bytes();
        let mut reader = ReferenceReader::new(&buf);

        let (this, _bits) = Self::decode(&mut reader, Some(buf.len() * 8)).context(VarIntSnafu)?;

        Ok(this)
    }
}
impl TryFrom<Token> for x!(..) {
    type Error = TokenError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        let mut writer = ReferenceWriter::new();
        value
            .encode(&mut writer, value.len_bits().ok())
            .context(VarIntSnafu)?;
        let buf = writer.finish().context(WriterSnafu)?;
        Ok(Self::from(buf))
    }
}

#[derive(Debug, Snafu, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TokenError {
    VarInt { source: varint::Error },
    Writer { source: WriterError },
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Token {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::builder().delete().alias(6u8).build();
            let b1 = vec![
                0, // delete type
                6, // alias
            ];
            let l1 = b1.len() * 8;

            let v2 = Self::builder()
                .register()
                .alias(3u8)
                .typ(10u8)
                .value([1, 2, 3])
                .build();
            let b2 = vec![
                1,  // register type
                3,  // alias
                10, // type
                1, 2, 3, // value
            ];
            let l2 = b2.len() * 8;

            let v3 = Self::builder().use_alias().alias(8u8).build();
            let b3 = vec![
                2, // use alias type
                8, // alias
            ];
            let l3 = b3.len() * 8;

            let v4 = Self::builder()
                .use_value()
                .typ(40u8)
                .value([10, 11, 12, 13])
                .build();
            let b4 = vec![
                3,  // use value type
                40, // type
                10, 11, 12, 13, // value
            ];
            let l4 = b4.len() * 8;

            vec![(v1, b1, l1), (v2, b2, l2), (v3, b3, l3), (v4, b4, l4)]
        }
    }

    varint_struct_test!(Token);
}
