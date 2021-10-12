use yew_agent::HandlerId;

#[derive(Debug)]
pub enum Msg<T> {
    Respond((T, HandlerId)),
}
