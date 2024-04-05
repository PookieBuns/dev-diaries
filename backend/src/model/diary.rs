use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::{FromRow, Type};

#[derive(Debug, Deserialize)]
pub struct Diary {
    pub user_id: i32,
    pub diary_date: Option<NaiveDate>,
    pub leet_code_problems: Vec<LeetCodeProblem>,
    pub job_applications: Vec<JobApplication>,
}

#[derive(Debug, Deserialize)]
pub struct LeetCodeProblem {
    pub problem_link: String,
    pub difficulty: DifficultyLevel,
    pub is_done: bool,
}

#[derive(Debug, Deserialize)]
pub struct JobApplication {
    pub company_name: String,
    pub job_application_link: String,
    pub is_done: bool,
}

#[derive(FromRow, Debug)]
pub struct DiaryDB {
    pub diary_id: Option<i32>,
    pub user_id: i32,
    pub diary_date: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Type, Deserialize)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(FromRow, Debug)]
pub struct LeetCodeProblemDB {
    pub leet_code_problem_id: Option<i32>,
    pub diary_id: i32,
    pub problem_link: String,
    pub difficulty: DifficultyLevel,
    pub is_done: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Debug)]
pub struct JobApplicationDB {
    pub job_application_id: Option<i32>,
    pub diary_id: i32,
    pub company_name: String,
    pub job_application_link: String,
    pub is_done: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
