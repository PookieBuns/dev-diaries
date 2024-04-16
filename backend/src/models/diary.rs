use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Type};

#[derive(Debug, Deserialize, Serialize)]
pub struct Diary {
    pub diary_id: Option<i32>,
    pub diary_date: NaiveDate,
    pub diary_notes: String,
    pub leet_code_problems: Vec<LeetCodeProblem>,
    pub job_applications: Vec<JobApplication>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDiary {
    pub user_id: i32,
    pub diary: Diary,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct LeetCodeProblem {
    pub diary_id: Option<i32>,
    pub problem_link: String,
    pub difficulty: DifficultyLevel,
    pub is_done: bool,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct JobApplication {
    pub diary_id: Option<i32>,
    pub company_name: String,
    pub job_application_link: String,
    pub is_done: bool,
}
