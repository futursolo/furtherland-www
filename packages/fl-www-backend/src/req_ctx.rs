use crate::resident::Resident;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub resident: Option<Resident>,
}
