use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

// use proc_macro2::{Ident, TokenStream};
// use quote::quote;
// use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput};
// use syn::{Item, ItemFn};

mod atom;
mod css_var;
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

#[proc_macro]
#[proc_macro_error]
pub fn css_var(input: TokenStream) -> TokenStream {
    css_var::macro_fn(input.into()).into()
}

#[proc_macro_derive(Atom)]
#[proc_macro_error]
pub fn atom(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    atom::macro_fn(input).into()
}
