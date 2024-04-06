mod models;

use super::DiaryRepo;
use crate::models::Diary;
use crate::Result;
use axum::async_trait;
use models::{PgDiary, PgJobApplication, PgLeetCodeProblem};
use sqlx::{PgPool, QueryBuilder, Row};

#[derive(Clone)]
pub struct PgDiaryRepo {
    pool: PgPool,
}

impl PgDiaryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DiaryRepo for PgDiaryRepo {
    async fn create(&self, diary: &Diary) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        let row = sqlx::query(
            "INSERT INTO \"diary\" (user_id, diary_date)
        VALUES ($1, $2) returning diary_id",
        )
        .bind(diary.user_id)
        .bind(diary.diary_date)
        .fetch_one(&mut *transaction)
        .await?;
        let diary_id: i32 = row.get("diary_id");
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO leet_code_problem (diary_id, problem_link, difficulty, is_done)",
        );
        if !diary.leet_code_problems.is_empty() {
            query_builder.push_values(diary.leet_code_problems.iter(), |mut builder, problem| {
                builder
                    .push_bind(diary_id)
                    .push_bind(&problem.problem_link)
                    .push_bind(&problem.difficulty)
                    .push_bind(problem.is_done);
            });
            let query = query_builder.build();
            query.execute(&mut *transaction).await?;
        }
        if !diary.job_applications.is_empty() {
            query_builder = QueryBuilder::new(
                "INSERT INTO job_application (diary_id, company_name, job_application_link, is_done)",
            );
            query_builder.push_values(diary.job_applications.iter(), |mut builder, application| {
                builder
                    .push_bind(diary_id)
                    .push_bind(&application.company_name)
                    .push_bind(&application.job_application_link)
                    .push_bind(application.is_done);
            });
            let query = query_builder.build();
            query.execute(&mut *transaction).await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn get(&self, user_id: i32) -> Result<Diary> {
        let diaries = sqlx::query_as!(PgDiary, "SELECT * FROM diary WHERE user_id = $1", user_id)
            .fetch_all(&self.pool)
            .await?;
        !todo!()
    }
}
