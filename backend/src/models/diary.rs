use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::Type;

#[derive(Debug, Deserialize)]
pub struct Diary {
    pub user_id: Option<i32>,
    pub diary_date: Option<NaiveDate>,
    pub leet_code_problems: Vec<LeetCodeProblem>,
    pub job_applications: Vec<JobApplication>,
}

#[derive(Debug, Type, Deserialize)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
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
