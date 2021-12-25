use crate::user::User;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user: Option<User>,
}
