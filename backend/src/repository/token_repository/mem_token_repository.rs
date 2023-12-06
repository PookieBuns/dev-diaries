use super::TokenRepo;
use crate::errors::Error;
use crate::Result;
use axum::async_trait;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct MemTokenRepo {
    tokens: Arc<Mutex<HashSet<String>>>,
}

impl MemTokenRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TokenRepo for MemTokenRepo {
    async fn create(&self, token: &str) -> Result<()> {
        let mut tokens = self.tokens.lock().await;
        tokens.insert(token.to_string());
        println!("tokens: {:?}", tokens);
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<()> {
        let tokens = self.tokens.lock().await;
        println!("tokens: {:?}", tokens);
        if tokens.contains(token) {
            Ok(())
        } else {
            Err(Error::Unknown)
        }
    }
}
