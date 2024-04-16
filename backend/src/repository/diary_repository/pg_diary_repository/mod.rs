mod models;

use super::DiaryRepo;
use crate::models::diary::{DifficultyLevel, JobApplication, LeetCodeProblem};
use crate::models::Diary;
use crate::Result;
use axum::async_trait;
use models::PgDiary;
use sqlx::{PgPool, QueryBuilder, Row};
use std::collections::HashMap;

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
            "INSERT INTO diary (user_id, diary_date)
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

    async fn get(&self, user_id: i32) -> Result<Vec<Diary>> {
        let pg_diaries =
            sqlx::query_as!(PgDiary, "SELECT * FROM diary WHERE user_id = $1", user_id)
                .fetch_all(&self.pool)
                .await?;
        let diary_ids: Vec<i32> = pg_diaries.iter().map(|diary| diary.diary_id).collect();
        let leet_code_problems = sqlx::query_as!(
            LeetCodeProblem,
            r#"
            SELECT
                diary_id,
                problem_link,
                difficulty as "difficulty: DifficultyLevel",
                is_done
            FROM
                leet_code_problem
            WHERE
                diary_id = ANY($1)
            "#,
            &diary_ids
        )
        .fetch_all(&self.pool)
        .await?;
        let job_applications = sqlx::query_as!(
            JobApplication,
            r#"
            SELECT
                diary_id,
                company_name,
                job_application_link,
                is_done
            FROM
                job_application
            WHERE
                diary_id = ANY($1)
            "#,
            &diary_ids
        )
        .fetch_all(&self.pool)
        .await?;
        let mut lee_code_problems_map: HashMap<i32, Vec<LeetCodeProblem>> = leet_code_problems
            .into_iter()
            .fold(HashMap::new(), |mut map, problem| {
                map.entry(problem.diary_id.unwrap())
                    .or_default()
                    .push(problem);
                map
            });
        let mut job_applications_map: HashMap<i32, Vec<JobApplication>> = job_applications
            .into_iter()
            .fold(HashMap::new(), |mut map, application| {
                map.entry(application.diary_id.unwrap())
                    .or_default()
                    .push(application);
                map
            });
        let diaries = pg_diaries.into_iter().map(|pg_diary| {
            let leet_code_problems = lee_code_problems_map
                .remove(&pg_diary.diary_id)
                .unwrap_or_default();
            let job_applications = job_applications_map
                .remove(&pg_diary.diary_id)
                .unwrap_or_default();
            Diary {
                diary_id: Some(pg_diary.diary_id),
                user_id: Some(pg_diary.user_id),
                diary_date: Some(pg_diary.diary_date),
                leet_code_problems,
                job_applications,
            }
        });
        Ok(diaries.collect())
    }
}
