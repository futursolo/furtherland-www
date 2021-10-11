use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, PartialEq, TypedBuilder)]
pub struct Client {
    #[builder(setter(into, strip_option))]
    base_url: Option<String>,
}

impl Client {
    pub fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }
}
