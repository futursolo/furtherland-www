use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields};

use proc_macro_error::abort_call_site;

use crate::utils::get_rand_str;

pub(crate) fn macro_fn(input: DeriveInput) -> TokenStream {
    let data = match input.data {
        Data::Struct(ref m) => m.to_owned(),
        _ => abort_call_site!("only structs are supported!"),
    };

    let fields = match data.fields {
        Fields::Named(ref m) => m.named.iter().cloned().collect::<Vec<Field>>(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => abort_call_site!("only named structs are supported at this moment!"),
    };

    let mut idents = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();
        idents.push(ident.to_string());
    }

    let entropy = Literal::string(&get_rand_str());

    let ident_strs = idents
        .iter()
        .map(|m| Literal::string(m))
        .collect::<Vec<_>>();

    let field_number = Literal::usize_unsuffixed(idents.len());

    let css_vars_impl = quote! {
        impl ::fl_www_common::styling::CssVariables {
            fn entropy() -> &'static str {
                #entropy
            }
            fn to_css_vars_nested_with_prefix(entropy: &str, prefix: &str, w: &mut ::std::collections::HashMap<String, String>) {
                let full_prefix = ;

                #( w.insert(format!("--stylist-{}-{}-{}", entropy, prefix, #ident_strs), self.#idents.to_string()); )*

            }
            fn to_css_vars_for(selector: &str) -> ::stylist::StyleSource<'static> {
                use ::stylist::ast::*;

                let mut vars = HashMap::with_capacity(#field_number);
            }
        }
    };

    todo!()
}
