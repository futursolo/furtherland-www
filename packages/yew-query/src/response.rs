use std::rc::Rc;

use web_sys::Headers;

#[derive(Debug, Clone)]
pub struct Response<T>
where
    T: Clone + 'static,
{
    // inner: Rc<reqwest::Response>,
    pub(crate) data: Rc<T>,
    pub(crate) headers: Headers,
}

impl<T> Response<T>
where
    T: Clone + 'static,
{
    pub fn data(&self) -> Rc<T> {
        self.data.clone()
    }

    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }
}
