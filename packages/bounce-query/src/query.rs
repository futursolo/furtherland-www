use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;

pub type QueryResult<T> = std::result::Result<Rc<T>, <T as Query>::Error>;

#[async_trait(?Send)]
pub trait Query: PartialEq {
    type Input: 'static;
    type Error: 'static + std::error::Error + Clone;

    async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self>;
}
