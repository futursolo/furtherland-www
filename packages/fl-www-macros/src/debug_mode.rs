use proc_macro2::{Literal, TokenStream};
use quote::quote;

use bincode::serialize_into;
use syntect::parsing::SyntaxSet;

pub(crate) fn emit_tokens() -> TokenStream {
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let mut v = Vec::new();
    serialize_into(&mut v, &syntax_set).expect("Failed to serialise encoder.");

    let dumped_tokens = Literal::byte_string(&v);

    let dumped_fn_tokens = quote! {
        || {
            use ::syntect::dumps::from_binary;
            use bincode::deserialize_from;

            let buf = #dumped_tokens;
            deserialize_from(buf.as_ref()).unwrap()
        }
    };

    quote! { ::once_cell::sync::Lazy::new(#dumped_fn_tokens) }
}
