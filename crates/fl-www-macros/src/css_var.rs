use std::iter::repeat;

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();

    let first_token = match tokens.next() {
        Some(m) => m,
        None => abort_call_site!("expect at least 1 token"),
    };

    let theme_ident = match first_token {
        TokenTree::Ident(m) => m,
        _ => abort!(first_token, "expected ident"),
    };

    let mut idents = vec![theme_ident.to_string().to_case(Case::Kebab)];

    let period_iter = repeat(true).interleave(repeat(false));

    for (token, should_be_period) in tokens.zip(period_iter) {
        if should_be_period {
            let maybe_punct = match token.clone() {
                TokenTree::Punct(m) => Some(m.as_char()),
                _ => None,
            };

            if maybe_punct != Some('.') {
                abort!(token, "expected period");
            }
        } else {
            let ident = match token {
                TokenTree::Ident(m) => m,
                _ => abort!(token, "expected ident"),
            };

            idents.push(ident.to_string().to_case(Case::Kebab));
        }
    }

    let name = idents.join("-");

    quote! { format!("var(--stylist-{}-{}, {})", #theme_ident.entropy(), #name, #input) }
}
