use convert_case::{Case, Casing};
use proc_macro_error2::abort;
use quote::{ToTokens, quote};
use syn::{Expr, Ident, Lit, parse::Parse, spanned::Spanned};

const BASE_URL: &str = "https://www.ietf.org/archive/id/draft-ietf-moq-transport";
const URL_EXT: &str = "html";

const VERSION: &str = "v";
const RENAME: &str = "rename";
const APPEND: &str = "append";
const ERROR: &str = "error";

#[derive(Debug)]
pub struct DraftRefArgs {
    /// draft version
    pub version: Lit,

    /// override of the fragment
    pub rename: Option<Lit>,

    /// append _error
    pub error: bool,

    /// append an arbitrary literal to the url
    pub append: Option<Lit>,
}

impl Parse for DraftRefArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut version: Option<Lit> = None;
        let mut rename = None;
        let mut append = None;
        let mut error = false;

        for expr in input.parse_terminated(Expr::parse, Token![,])? {
            match &expr {
                Expr::Assign(assign) => match assign.left.to_token_stream().to_string().as_str() {
                    VERSION => version = Some(syn::parse(assign.right.to_token_stream().into())?),
                    RENAME => rename = Some(syn::parse(assign.right.to_token_stream().into())?),
                    APPEND => append = Some(syn::parse(assign.right.to_token_stream().into())?),
                    x => abort!(expr.span(), "unknown assign {}", x),
                },
                Expr::Path(path) => error = path.path.is_ident(ERROR),
                expr => abort!(
                    expr.span(),
                    "unknown argument, only assign and path are supported"
                ),
            }
        }

        Ok(Self {
            version: version
                .unwrap_or_else(|| abort!(input.span(), "missing version argument \"v\"")),
            rename,
            error,
            append,
        })
    }
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
