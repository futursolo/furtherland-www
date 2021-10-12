use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Item, ItemFn};

#[proc_macro_attribute]
#[proc_macro_error]
fn task(attrs: TokenStream, code: TokenStream) -> TokenStream {
    todo!()
}
