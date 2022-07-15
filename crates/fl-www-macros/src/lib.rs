use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod css_var;
mod css_variables;
mod utils;

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
