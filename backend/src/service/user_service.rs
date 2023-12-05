use crate::errors::Error;
use crate::model::User;
use crate::repository::user_repository::UserRepo;
use crate::Result;
use crate::auth::{hash_password, verify_password, generate_jwt};



#[derive(Clone)]
pub struct UserService<T>
where
    T: UserRepo,
{
    user_repository: T,
}

impl<T> UserService<T>
where
    T: UserRepo,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<()> {
        let password_hash = hash_password(password)?;
        let user = User {
            user_id: None,
            user_name: username.to_string(),
            password: password_hash,
        };
        self.user_repository.create(&user).await?;
        Ok(())
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String> {
        let user = self.user_repository.find_by_username(username).await?;
        if !verify_password(password, &user.password.salt, &user.password.hash) {
            return Err(Error::Auth);
        }
        let jwt = generate_jwt(&user.user_name, user.user_id.unwrap())?;
        Ok(jwt)
    }
}
