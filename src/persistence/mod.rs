//! Contains the trait needed for implementing a database access object as well
//! as implementations.

use std::convert::TryInto;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::types::Uuid;
use crate::models::prelude::*;

/// The interface for any database access object that will interact with the the questions database.
pub trait QuestionDao {
    /// # Required Method
    /// Creates a new question and inserts it into the database.
    ///
    /// # Parameters
    /// `new_question`: The content of the new question to be created and inserted into the database
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the question was created successfully a `Ok(Uuid)` will be returned
    /// where the `Uuid` represents the id of the newly created question, otherwise `Err(DbError)` will be returned.
    async fn create_question(&self, new_question: NewQuestion) -> Result<Uuid, DbError>;

    /// # Required Method
    /// Gets a question from the database if present.
    ///
    /// # Parameters
    /// `question_id` the `EntityId` of the question being queried
    ///
    ///# Returns
    /// A `Result<Question, DbError>`, a `Ok(Question)` if the query is successful, otherwise `Err(DbError)`.
    async fn get_question(&self, question_id: EntityId) -> Result<Question, DbError>;

    /// # Required Method
    /// Gets a `Vec` of all questions in the database
    ///
    /// # Returns
    /// A `Result<Vec<Question>>, DbError>`, in the success case `Ok(Vec<Question>)`, otherwise `Err(DbError)`.
    async fn get_questions(&self, ) -> Result<Vec<Question>, DbError>;

    /// # Required Method
    /// Deletes a question from the database.
    ///
    /// # Parameters
    /// `question_id` the `EntityId` of the `Question` to be deleted.
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the question is successfully deleted then a `Ok(Uuid)` will be returned,
    /// otherwise an `Err(DbError)` is returned.
    async fn delete_question(&self, question_id: EntityId) -> Result<Uuid, DbError>;

    /// # Required Method
    /// Increments the number of likes associated with a particular question
    ///
    /// # Parameters
    /// `question_id`, the `EntityId` of the `Question` being queried
    ///
    /// # Returns
    /// A `Result<(), DbError>`, `Ok(())` in the successful case and `Err(DbError)` in the
    /// unsuccessful case.
    async fn increment_question_likes(&self, question_id: EntityId) -> Result<(), DbError>;
}

/// The interface for any database access object that will interact with the answers database.
pub trait AnswerDao {
    /// # Required Method
    /// Creates a new answer for a particular question and inserts it into the database.
    ///
    /// # Parameters
    /// `question_id`: The `EntityId` of the new answer is attempting to respond to
    /// `new_answer`: The `NewAnswer` containing the content of the answer to be inserted into the database
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the answer was created successfully a `Ok(Uuid)` will be returned
    /// where the `Uuid` represents the id of the newly created answer, otherwise `Err(DbError)` will be returned.
    async fn create_answer(&self, question_id: EntityId, new_answer: NewAnswer) -> Result<Uuid, DbError>;

    /// # Required Method
    /// Gets an answer from the database if present
    ///
    /// # Parameters
    /// `answer_id` the `EntityId` of the answer being queried
    ///
    ///# Returns
    /// A `Result<Answer, DbError>`, a `Ok(Question)` if the query is successful, otherwise `Err(DbError)`.
    async fn get_answer(&self, answer_id: EntityId) -> Result<Answer, DbError>;

    /// # Required Method
    /// Gets a `Vec` of all answers in the database associated with a particular question.
    ///
    /// # Parameters
    /// `question_id`: The id of the `Question` whose answers are to be returned.
    ///
    /// # Returns
    /// A `Result<Vec<Answer>>, DbError>`, in the success case `Ok(Vec<Answer>)`, otherwise `Err(DbError)`.
    async fn get_answers(&self, question_id: EntityId) -> Result<Vec<Answer>, DbError>;

    /// # Required Method
    /// Gets a `Vec` of all answers in the database
    ///
    /// # Returns
    /// A `Result<Vec<Answer>>, DbError>`, in the success case `Ok(Vec<Question>)`, otherwise `Err(DbError)`.
    async fn get_all_answers(&self) -> Result<Vec<Answer>, DbError>;

    /// # Required Method
    /// Deletes an answer from the database.
    ///
    /// # Parameters
    /// `answer_id`: The `EntityId` of the `Answer` to be deleted.
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the answer is successfully deleted then a `Ok(Uuid)` will be returned,
    /// otherwise an `Err(DbError)` is returned.
    async fn delete_answer(&self, answer_id: EntityId) -> Result<Uuid, DbError>;

    /// # Required Method
    /// Increments the number of likes associated with a particular answer.
    ///
    /// # Parameters
    /// `answer_id`: The `EntityId` of the `Answer` being queried
    ///
    /// # Returns
    /// A `Result<(), DbError>`, `Ok(())` in the successful case and `Err(DbError)` in the
    /// unsuccessful case.
    async fn increment_answer_likes(&self, answer_id: EntityId) -> Result<(), DbError>;
}

pub struct QuestionDaoImpl {
    pool: PgPool,
}

impl QuestionDao for QuestionDaoImpl {
    async fn create_question(&self, new_question: NewQuestion) -> Result<Uuid, DbError> {
        sqlx::query("INSERT INTO questions (title, question) VALUES ($1, $2) returning id")
            .bind(new_question.title)
            .bind(new_question.question)
            .map(|row: PgRow| -> Uuid { row.get("id") })
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::Creation(e))
    }

    async fn get_question(&self, question_id: EntityId) -> Result<Question, DbError> {
        let question_id: Uuid = question_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(question_id)
            .map(|row: PgRow| Question::new(
                row.get("id"),
                row.get("title"),
                row.get("question"),
                row.get("likes"),
                row.get("created_at")
            ))
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::NotFound(e))
    }

    async fn get_questions(&self) -> Result<Vec<Question>, DbError> {
        todo!()
    }

    async fn delete_question(&self, question_id: EntityId) -> Result<Uuid, DbError> {
        todo!()
    }

    async fn increment_question_likes(&self, question_id: EntityId) -> Result<(), DbError> {
        todo!()
    }
}

