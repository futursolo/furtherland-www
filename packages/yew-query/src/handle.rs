use crate::error::Result;
use crate::response::Response;

#[derive(Debug)]
pub struct UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    pub(crate) result: Option<Result<Response<T>, E>>,
}

impl<T, E> UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    pub fn result(&self) -> Option<Result<Response<T>, E>> {
        self.result.clone()
    }
}

impl<T, E> Clone for UseFetchHandle<T, E>
where
    T: Clone + 'static,
    E: std::error::Error + 'static,
{
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
        }
    }
}
