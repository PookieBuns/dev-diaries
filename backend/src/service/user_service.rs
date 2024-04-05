use crate::auth::{decode_jwt, generate_jwt, hash_password, verify_password};
use crate::errors::Error;
use crate::model::User;
use crate::repository::token_repository::TokenRepo;
use crate::repository::user_repository::UserRepo;
use crate::Result;

#[derive(Clone)]
pub struct UserService<T, U>
where
    T: UserRepo,
    U: TokenRepo,
{
    user_repository: T,
    token_repository: U,
}

impl<T, U> UserService<T, U>
where
    T: UserRepo,
    U: TokenRepo,
{
    pub fn new(user_repository: T, token_repository: U) -> Self {
        Self {
            user_repository,
            token_repository,
        }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<()> {
        let password_hash = hash_password(password)?;
        let user = User {
            user_id: None,
            user_name: username.to_string(),
            password: password_hash,
            created_at: None,
            updated_at: None,
            deleted_at: None,
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

    pub async fn request_password_reset(&self, username: &str) -> Result<()> {
        let user = self.user_repository.find_by_username(username).await?;
        let token = generate_jwt(&user.user_name, user.user_id.unwrap())?;
        println!("token: {}", token);
        self.token_repository.create(&token).await?;
        Ok(())
    }

    pub async fn verify_password_reset(&self, token: &str) -> Result<()> {
        self.token_repository.find_by_token(token).await?;
        decode_jwt(token)?;
        Ok(())
    }
}
