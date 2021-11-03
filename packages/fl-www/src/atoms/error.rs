use bounce::Atom;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) enum ErrorKind {
    Server,
    // Unknown,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct ErrorState {
    inner: Option<ErrorKind>,
}
impl ErrorState {
    pub fn kind(&self) -> Option<ErrorKind> {
        self.inner
    }
}

impl From<ErrorKind> for ErrorState {
    fn from(m: ErrorKind) -> Self {
        Self { inner: Some(m) }
    }
}

impl Atom for ErrorState {}
