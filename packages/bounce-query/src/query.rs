use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;

use async_trait::async_trait;
use bounce::prelude::*;
use yew::prelude::*;

use crate::utils::Id;

pub type QueryResult<T> = std::result::Result<Rc<T>, <T as Query>::Error>;

#[async_trait(?Send)]
pub trait Query: PartialEq {
    type Input: Hash + Eq + 'static;
    type Error: 'static + std::error::Error + PartialEq + Clone;

    async fn query(states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self>;
}

struct RunQueryInput<T>
where
    T: Query + 'static,
{
    id: Id,
    input: Rc<T::Input>,
}

impl<T> Hash for RunQueryInput<T>
where
    T: Query + 'static,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.input.hash(state);
    }
}

impl<T> PartialEq for RunQueryInput<T>
where
    T: Query + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id && self.input == rhs.input
    }
}
impl<T> Eq for RunQueryInput<T> where T: Query + 'static {}

#[derive(PartialEq)]
struct IsCurrentQuery<T>
where
    T: Query + 'static,
{
    _marker: PhantomData<T>,
    inner: bool,
}

impl<T> InputSelector for IsCurrentQuery<T>
where
    T: Query + 'static,
{
    type Input = RunQueryInput<T>;

    fn select(states: &BounceStates, input: Rc<RunQueryInput<T>>) -> Rc<Self> {
        if let Some(m) = states
            .get_slice_value::<QueryState<T>>()
            .queries
            .get(&input.input)
        {
            let id = match m {
                QueryStateStatus::Loading(id) => id,
                QueryStateStatus::Completed((id, _)) => id,
            };

            return Self {
                _marker: PhantomData,
                inner: *id == input.id,
            }
            .into();
        }

        Self {
            _marker: PhantomData,
            inner: false,
        }
        .into()
    }
}

#[future_notion(RunQuery)]
async fn run_query<T>(
    states: &BounceStates,
    input: Rc<RunQueryInput<T>>,
) -> Rc<Option<QueryResult<T>>>
where
    T: Query + 'static,
{
    let is_current_query = states.get_input_selector_value::<IsCurrentQuery<T>>(input.clone());

    if !is_current_query.inner {
        return None.into();
    }

    let result = T::query(states, input.input.clone()).await;

    Some(result).into()
}

#[derive(PartialEq)]
pub enum QueryStateStatus<T>
where
    T: Query + 'static,
{
    Loading(Id),
    Completed((Id, QueryResult<T>)),
}

impl<T> Clone for QueryStateStatus<T>
where
    T: Query + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Loading(ref id) => Self::Loading(*id),
            Self::Completed(ref m) => Self::Completed(m.clone()),
        }
    }
}

#[derive(Slice)]
#[with_notion(Deferred<RunQuery<T>>)]
struct QueryState<T>
where
    T: Query + 'static,
{
    ctr: u64,
    queries: HashMap<Rc<T::Input>, QueryStateStatus<T>>,
}

impl<T> Reducible for QueryState<T>
where
    T: Query + 'static,
{
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        self
    }
}

impl<T> Default for QueryState<T>
where
    T: Query + 'static,
{
    fn default() -> Self {
        Self {
            ctr: 0,
            queries: HashMap::new(),
        }
    }
}

impl<T> PartialEq for QueryState<T>
where
    T: Query + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

impl<T> WithNotion<Deferred<RunQuery<T>>> for QueryState<T>
where
    T: Query + 'static,
{
    fn apply(self: Rc<Self>, notion: Rc<Deferred<RunQuery<T>>>) -> Rc<Self> {
        match *notion {
            Deferred::Pending { ref input } => {
                if self.queries.contains_key(&input.input) {
                    return self;
                }

                let mut queries = self.queries.clone();
                queries.insert(input.input.clone(), QueryStateStatus::Loading(input.id));

                Self {
                    ctr: self.ctr + 1,
                    queries,
                }
                .into()
            }
            Deferred::Completed {
                ref input,
                ref output,
            } => {
                if let Some(ref output) = **output {
                    let mut queries = self.queries.clone();
                    queries.insert(
                        input.input.clone(),
                        QueryStateStatus::Completed((input.id, (*output).clone())),
                    );

                    Self {
                        ctr: self.ctr + 1,
                        queries,
                    }
                    .into()
                } else {
                    self
                }
            }
            Deferred::Outdated { ref input } => {
                if let Some(QueryStateStatus::Completed((ref m, _))) =
                    self.queries.get(&input.input)
                {
                    if m == &input.id {
                        return self;
                    }
                }

                let mut queries = self.queries.clone();
                queries.remove(&input.input);

                Self {
                    ctr: self.ctr + 1,
                    queries,
                }
                .into()
            }
        }
    }
}

#[derive(PartialEq)]
struct QuerySelector<T>
where
    T: Query + 'static,
{
    value: Option<QueryStateStatus<T>>,
}

impl<T> InputSelector for QuerySelector<T>
where
    T: Query + 'static,
{
    type Input = RunQueryInput<T>;

    fn select(states: &BounceStates, input: Rc<RunQueryInput<T>>) -> Rc<Self> {
        let value = states
            .get_slice_value::<QueryState<T>>()
            .queries
            .get(&input.input)
            .cloned();

        Self { value }.into()
    }
}

#[derive(Clone)]
pub struct UseQueryValueHandle<T>
where
    T: Query + 'static,
{
    _value: Option<QueryStateStatus<T>>,
}

pub fn use_query_value<T>(input: Rc<T::Input>) -> UseQueryValueHandle<T>
where
    T: Query + 'static,
{
    let id = *use_ref(Id::new);
    let input = Rc::new(RunQueryInput::<T> { id, input });
    let value = use_input_selector_value::<QuerySelector<T>>(input.clone());
    let run_query = use_future_notion_runner::<RunQuery<T>>();

    use_effect_with_deps(
        move |input| {
            run_query(input.clone());

            || {}
        },
        input,
    );

    UseQueryValueHandle {
        _value: value.value.clone(),
    }
}
