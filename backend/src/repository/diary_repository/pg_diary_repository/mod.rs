mod models;

use super::DiaryRepo;
use crate::models::diary::UserDiary;
use crate::models::diary::{DifficultyLevel, JobApplication, LeetCodeProblem};
use crate::models::Diary;
use crate::Result;
use axum::async_trait;
use models::PgDiary;
use sqlx::{PgPool, QueryBuilder};
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
    async fn create(&self, user_diary: &UserDiary) -> Result<i32> {
        let diary = &user_diary.diary;
        let mut transaction = self.pool.begin().await?;

        // Insert or Update diary
        let diary_id = match diary.diary_id {
            None => {
                sqlx::query!(
                    "INSERT INTO diary (user_id, diary_date, diary_notes) VALUES ($1, $2, $3) RETURNING diary_id",
                    user_diary.user_id,
                    diary.diary_date,
                    diary.diary_notes
                )
                .fetch_one(&mut *transaction)
                .await?
                .diary_id
            },
            Some(diary_id) => {
                sqlx::query!(
                    "UPDATE diary SET diary_date = $1, diary_notes = $2 WHERE diary_id = $3",
                    diary.diary_date,
                    diary.diary_notes,
                    diary_id
                )
                .execute(&mut *transaction)
                .await?;
                diary_id
            },
        };

        // Replace leet_code_problems
        sqlx::query!(
            "UPDATE leet_code_problem SET deleted_at = NOW() WHERE diary_id = $1",
            diary_id
        )
        .execute(&mut *transaction)
        .await?;

        if !diary.leet_code_problems.is_empty() {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO leet_code_problem (diary_id, problem_link, difficulty, is_done)",
            );
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
        // Replace job_applications
        sqlx::query!(
            "UPDATE job_application SET deleted_at = NOW() WHERE diary_id = $1",
            diary_id
        )
        .execute(&mut *transaction)
        .await?;

        if !diary.job_applications.is_empty() {
            let mut query_builder = QueryBuilder::new(
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

        // Commit transaction
        transaction.commit().await?;
        Ok(diary_id)
    }

    async fn get(&self, user_id: i32) -> Result<Vec<UserDiary>> {
        let pg_diaries = sqlx::query_as!(
            PgDiary,
            "SELECT * FROM diary WHERE user_id = $1 AND deleted_at is NULL order by diary_date DESC, diary_id",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        let diary_ids: Vec<i32> = pg_diaries.iter().map(|diary| diary.diary_id).collect();
        let leet_code_problems = sqlx::query_as!(
            LeetCodeProblem,
            r#"
            SELECT
                leet_code_problem_id,
                diary_id,
                problem_link,
                difficulty as "difficulty: DifficultyLevel",
                is_done
            FROM
                leet_code_problem
            WHERE
                diary_id = ANY($1) AND deleted_at IS NULL
            "#,
            &diary_ids
        )
        .fetch_all(&self.pool)
        .await?;
        let job_applications = sqlx::query_as!(
            JobApplication,
            r#"
            SELECT
                job_application_id,
                diary_id,
                company_name,
                job_application_link,
                is_done
            FROM
                job_application
            WHERE
                diary_id = ANY($1) AND deleted_at IS NULL
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
            let diary = Diary {
                diary_id: Some(pg_diary.diary_id),
                diary_date: pg_diary.diary_date,
                diary_notes: pg_diary.diary_notes,
                leet_code_problems,
                job_applications,
            };
            UserDiary {
                user_id: pg_diary.user_id,
                diary,
            }
        });
        Ok(diaries.collect())
    }
}
