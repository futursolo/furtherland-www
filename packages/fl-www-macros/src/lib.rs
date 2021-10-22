use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};

// use proc_macro2::{Ident, TokenStream};
// use quote::quote;
// use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput};
// use syn::{Item, ItemFn};

mod css_variables;
mod utils;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn task(_attrs: TokenStream, _code: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(CssVariables, attributes(css_vars))]
#[proc_macro_error]
pub fn css_variables(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    css_variables::macro_fn(input).into()
}
