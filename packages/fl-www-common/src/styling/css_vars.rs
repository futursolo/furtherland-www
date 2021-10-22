use std::collections::HashMap;
use stylist::StyleSource;

pub trait CssVariables {
    fn entropy() -> &'static str;
    fn to_css_vars_nested_with_prefix(entropy: &str, prefix: &str, w: &mut HashMap<String, String>);
    fn to_css_vars_for(selector: &str) -> StyleSource<'static>;
}
