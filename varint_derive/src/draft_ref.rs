use convert_case::{Case, Casing};
use darling::FromMeta;
use quote::{ToTokens, quote};
use syn::{Ident, Lit};

const BASE_URL: &str = "https://www.ietf.org/archive/id/draft-ietf-moq-transport";
const URL_EXT: &str = "html";

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct DraftRefArgs {
    /// draft version
    #[darling(rename = "v")]
    pub version: Lit,

    /// override of the fragment
    pub rename: Option<Lit>,

    /// append _error
    #[darling(default)]
    pub error: bool,

    /// append an arbitrary literal to the url
    pub append: Option<Lit>,
}

impl DraftRefArgs {
    pub fn to_doc_string(&self, name: &Ident) -> String {
        let version = &self.version;

        let mut frag = format!("name-{}", name.to_string().to_case(Case::Snake));

        match self {
            Self {
                rename: Some(rename),
                error,
                ..
            } => {
                frag = if *error {
                    format!("{}_error", Self::token_to_string(rename))
                } else {
                    Self::token_to_string(rename)
                }
            }
            Self { error: true, .. } => frag.push_str("_error"),
            _ => {}
        }

        if let Self {
            append: Some(s), ..
        } = self
        {
            frag.push_str(&Self::token_to_string(s));
        }

        let url = format!("{BASE_URL}-{}.{URL_EXT}#{frag}", quote! { #version });
        format!("\n\nFull details can be found in the [Draft]({url}).")
    }

    fn token_to_string<T>(token: &T) -> String
    where
        T: ToTokens,
    {
        token
            .to_token_stream()
            .to_string()
            .trim_matches('"')
            .to_string()
    }
}
