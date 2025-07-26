use crate::web::user::User;

#[derive(Clone)]
pub struct Ctx {
    pub user: User,
}
