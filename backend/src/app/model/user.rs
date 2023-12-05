use ring::digest;

#[derive(sqlx::FromRow, Debug)]
pub struct PasswordHash {
    pub salt: [u8; digest::SHA256_OUTPUT_LEN],
    #[sqlx(rename = "password_hash")]
    pub hash: [u8; digest::SHA256_OUTPUT_LEN],
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub user_id: Option<i32>,
    pub user_name: String,
    #[sqlx(flatten)]
    pub password: PasswordHash,
}
