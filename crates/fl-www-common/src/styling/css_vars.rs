use std::collections::HashMap;

use stylist::StyleSource;

pub trait CssVariables {
    fn entropy(&self) -> &'static str;
    fn to_css_vars_nested_with_prefix(
        &self,
        entropy: &str,
        prefix: &str,
        w: &mut HashMap<String, String>,
    );
    fn to_css_vars_for(&self, selector: &str) -> StyleSource<'static>;
}
