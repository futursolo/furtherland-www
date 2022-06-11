use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident};

use crate::utils::get_rand_str;

pub(crate) fn macro_fn(input: DeriveInput) -> TokenStream {
    let data = match input.data {
        Data::Struct(ref m) => m.to_owned(),
        _ => abort_call_site!("only structs are supported!"),
    };

    let struct_ident = input.ident;
    let struct_ident_str = Literal::string(&struct_ident.to_string().to_case(Case::Kebab));

    let fields = match data.fields {
        Fields::Named(ref m) => m.named.iter().cloned().collect::<Vec<Field>>(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => abort_call_site!("only named structs are supported at this moment!"),
    };

    let mut idents = Vec::new();
    let mut nested_idents = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();

        let css_var_attrs = field
            .attrs
            .iter()
            .filter(|m| m.path.get_ident().map(|m| m.to_string()).as_deref() == Some("css_vars"))
            .cloned()
            .collect::<Vec<_>>();

        if css_var_attrs.len() > 1 {
            abort!(
                css_var_attrs.get(1).unwrap(),
                "only 1 #[css_vars] attribute is allowed."
            );
        }

        let kind = if css_var_attrs.len() == 1 {
            let css_var_attr = css_var_attrs.first().cloned().unwrap();

            Some(match css_var_attr.parse_args::<Ident>() {
                Ok(m) => m,
                Err(e) => return e.to_compile_error(),
            })
        } else {
            None
        };

        let kind_str = kind.clone().map(|m| m.to_string());

        match (kind, kind_str.as_deref()) {
            (_, Some("nested")) => nested_idents.push(ident),
            (_, Some("skipped")) => {}
            (_, None) => idents.push(ident),
            (Some(m), Some(s)) => abort!(m, "unknown kind: {}", s),
            _ => unreachable!(),
        }
    }

    let entropy = Literal::string(&get_rand_str());

    let ident_strs = idents
        .iter()
        .map(|m| Literal::string(&m.to_string().to_case(Case::Kebab)))
        .collect::<Vec<_>>();

    let nested_ident_strs = nested_idents
        .iter()
        .map(|m| Literal::string(&m.to_string().to_case(Case::Kebab)))
        .collect::<Vec<_>>();

    let field_number = Literal::usize_unsuffixed(idents.len());

    let mut ident_inserts = TokenStream::new();
    for (ident, ident_str) in idents.iter().zip(ident_strs.iter()) {
        let stmt = quote! {
            w.insert(format!("--stylist-{}-{}-{}", entropy, prefix, #ident_str), self.#ident.to_string());
        };

        ident_inserts.extend(stmt);
    }

    for (nested_ident, nested_ident_str) in nested_idents.iter().zip(nested_ident_strs.iter()) {
        let stmt = quote! {
            {
                let nested_prefix = format!("{}-{}", prefix, #nested_ident_str);
                self.#nested_ident.to_css_vars_nested_with_prefix(entropy, &nested_prefix, w);
            }
        };

        ident_inserts.extend(stmt);
    }

    let css_vars_impl = quote! {
        #[automatically_derived]
        impl crate::styling::CssVariables for #struct_ident {
            fn entropy(&self) -> &'static str {
                #entropy
            }

            fn to_css_vars_nested_with_prefix(&self, entropy: &str, prefix: &str, w: &mut ::std::collections::HashMap<String, String>) {
                #ident_inserts
            }

            fn to_css_vars_for(&self, selector: &str) -> ::stylist::StyleSource<'static> {
                use ::stylist::ast::*;
                use ::std::collections::HashMap;

                let mut vars = HashMap::with_capacity(#field_number);

                self.to_css_vars_nested_with_prefix(self.entropy(), #struct_ident_str, &mut vars);

                let mut contents = Vec::new();

                for (name, value) in vars.into_iter() {
                    let val = StyleAttribute {
                        key: name.into(),
                        value: vec![value.into()].into(),
                    };

                    let content = RuleBlockContent::StyleAttr(val);

                    contents.push(content);
                }

                let block = Block {
                    condition: vec![vec![selector.to_owned().into()].into()].into(),
                    content: contents.into(),
                };

                let scope_contents = vec![ScopeContent::Block(block)];

                Sheet::from(scope_contents).into()
            }
        }
    };

    quote! {
        #css_vars_impl
    }
}
