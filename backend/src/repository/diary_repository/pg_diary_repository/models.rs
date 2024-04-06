use crate::models::diary::DifficultyLevel;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct PgDiary {
    pub diary_id: i32,
    pub user_id: i32,
    pub diary_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Debug)]
pub struct PgLeetCodeProblem {
    pub leet_code_problem_id: i32,
    pub diary_id: i32,
    pub problem_link: String,
    pub difficulty: DifficultyLevel,
    pub is_done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Debug)]
pub struct PgJobApplication {
    pub job_application_id: i32,
    pub diary_id: i32,
    pub company_name: String,
    pub job_application_link: String,
    pub is_done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
