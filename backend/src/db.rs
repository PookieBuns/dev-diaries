use crate::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use url::Url;

fn database_url() -> String {
    let mut url = Url::parse("postgres://").unwrap();
    url.set_host(Some("localhost")).expect("host is not valid");
    url.set_port(Some(5432)).expect("port is not valid");
    url.set_username("user").expect("username is not valid");
    url.set_password(Some("password"))
        .expect("password is not valid");
    url.set_path("/postgres");
    url.into()
}

pub async fn db_pool() -> Result<PgPool> {
    let database_url = database_url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}
