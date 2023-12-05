use super::UserRepo;
use crate::model::User;
use crate::Result;
use axum::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn find_by_username(&self, username: &str) -> Result<User> {
        let user: User = sqlx::query_as("SELECT * FROM \"user\" WHERE user_name = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<()> {
        sqlx::query(
            "INSERT INTO \"user\" (user_name, password_hash, salt)
        VALUES ($1, $2, $3)",
        )
        .bind(&user.user_name)
        .bind(user.password.hash)
        .bind(user.password.salt)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
