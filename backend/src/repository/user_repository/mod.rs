mod pg_user_repository;

use crate::model::User;
use crate::Result;
use axum::async_trait;
pub use pg_user_repository::PgUserRepo;

#[async_trait]
pub trait UserRepo {
    async fn find_by_username(&self, username: &str) -> Result<User>;
    async fn create(&self, user: &User) -> Result<()>;
}
