use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) enum ErrorKind {
    Server,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ErrorState {
    inner: UseEqualStateHandle<Option<ErrorKind>>,
}

impl ErrorState {
    pub fn set(&self, next_kind: ErrorKind) {
        self.inner.set(Some(next_kind));
    }

    pub fn kind(&self) -> Option<ErrorKind> {
        *self.inner.borrow()
    }
}

pub(crate) fn use_error_state() -> ErrorState {
    use_context::<ErrorState>().unwrap()
}

#[function_component(ErrorProvider)]
pub(crate) fn error_provider(props: &ChildrenProps) -> Html {
    let error_kind = use_equal_state(|| -> Option<ErrorKind> { None });

    let children = props.children.clone();

    let state = ErrorState { inner: error_kind };

    html! {
        <ContextProvider<ErrorState> context={state}>
            {children}
        </ContextProvider<ErrorState>>
    }
}
