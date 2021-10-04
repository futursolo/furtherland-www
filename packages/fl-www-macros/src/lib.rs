use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};

#[cfg(debug_assertions)]
mod debug_mode;

#[cfg(not(debug_assertions))]
fn emit_tokens_release() -> proc_macro2::TokenStream {
    use quote::quote;

    (quote! { ::once_cell::sync::Lazy::new(::syntect::parsing::SyntaxSet::load_defaults_newlines) })
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fl_syntax_set(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        abort_call_site!("This macro does not accept any tokens!");
    }

    #[cfg(debug_assertions)]
    return debug_mode::emit_tokens().into();

    #[cfg(not(debug_assertions))]
    emit_tokens_release().into()
}
