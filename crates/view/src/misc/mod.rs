use crate::prelude::*;

pub(crate) mod highlight;
pub(crate) mod markdown;

pub(crate) trait ToHtml {
    fn to_html(&self) -> Html;
}
