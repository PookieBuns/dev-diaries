-- Drop Tables and Types
DROP TABLE IF EXISTS job_application;
DROP TABLE IF EXISTS leet_code_problem;
DROP TABLE IF EXISTS diary;
DROP TABLE IF EXISTS user_account;
DROP TYPE IF EXISTS difficulty_level;

-- Create Tables and Types
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW(); -- Sets updated_at to current date and time
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE
    user_account
(
    user_id       SERIAL PRIMARY KEY,
    user_name     TEXT      NOT NULL UNIQUE,
    salt          BYTEA     NOT NULL,
    password_hash BYTEA     NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMP
);

CREATE TRIGGER update_user_updated_at
    BEFORE UPDATE
    ON user_account
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE
    diary
(
    diary_id    SERIAL PRIMARY KEY,
    user_id     INT       NOT NULL,
    diary_date  DATE      NOT NULL,
    diary_notes TEXT      NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_account (user_id) ON DELETE CASCADE
);

CREATE TRIGGER update_diary_updated_at
    BEFORE UPDATE
    ON diary
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TYPE difficulty_level AS ENUM ('easy', 'medium', 'hard');

CREATE TABLE
    leet_code_problem
(
    leet_code_problem_id SERIAL PRIMARY KEY,
    diary_id             INT              NOT NULL,
    problem_link         TEXT             NOT NULL,
    difficulty           difficulty_level NOT NULL,
    is_done              BOOLEAN          NOT NULL,
    created_at           TIMESTAMP        NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMP        NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMP,
    FOREIGN KEY (diary_id) REFERENCES diary (diary_id) ON DELETE CASCADE
);

CREATE TRIGGER update_leet_code_problem_updated_at
    BEFORE UPDATE
    ON leet_code_problem
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE
    job_application
(
    job_application_id   SERIAL PRIMARY KEY,
    diary_id             INT       NOT NULL,
    company_name         TEXT      NOT NULL,
    job_application_link TEXT      NOT NULL,
    is_done              BOOLEAN   NOT NULL,
    created_at           TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMP,
    FOREIGN KEY (diary_id) REFERENCES diary (diary_id) ON DELETE CASCADE
);

CREATE TRIGGER update_job_application_updated_at
    BEFORE UPDATE
    ON job_application
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE leet_code_submission
(
    leet_code_submission_id BIGSERIAL PRIMARY KEY,
    user_id                 INT       NOT NULL,
    is_pending              TEXT      NOT NULL,
    lang                    TEXT      NOT NULL,
    memory                  TEXT,
    runtime                 TEXT,
    status_display          TEXT      NOT NULL,
    time                    TEXT      NOT NULL,
    timestamp               BIGINT    NOT NULL,
    title                   TEXT      NOT NULL,
    title_slug              TEXT      NOT NULL,
    url                     TEXT      NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_account (user_id) ON DELETE CASCADE
);

CREATE TRIGGER update_submissions_updated_at
    BEFORE UPDATE
    ON leet_code_submission
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
