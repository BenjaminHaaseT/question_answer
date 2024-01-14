//! Contains the trait needed for implementing a database access object as well
//! as implementations.

use std::convert::TryInto;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::types::Uuid;
use sqlx::FromRow;
use crate::models::prelude::*;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use super::*;
}

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
    /// `new_answer`: The `NewAnswer` containing the content of the answer to be inserted into the database
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the answer was created successfully a `Ok(Uuid)` will be returned
    /// where the `Uuid` represents the id of the newly created answer, otherwise `Err(DbError)` will be returned.
    async fn create_answer(&self, new_answer: NewAnswer) -> Result<Uuid, DbError>;

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

impl QuestionDaoImpl {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
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
        // Attempt to parse entity id
        let question_id: Uuid = question_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id = $1")
            .bind(question_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::NotFound(e))
    }

    async fn get_questions(&self) -> Result<Vec<Question>, DbError> {
        sqlx::query("SELECT * FROM questions")
            .map(|row| Question::from_row(&row).map_err(|e| DbError::FromRow(e)))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::Access(e))?
            .into_iter()
            .collect::<Result<Vec<Question>, DbError>>()
    }

    async fn delete_question(&self, question_id: EntityId) -> Result<Uuid, DbError> {
        // Attempt to parse entity id
        let question_id: Uuid = question_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        let mut tx = self.pool.begin().await.map_err(|e| DbError::Access(e))?;
        // Ensure that a record with the given id exists
        sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(question_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| DbError::NotFound(e))?;
        // Now attempt to delete the record, and commit the changes if successful
        match sqlx::query("DELETE FROM questions WHERE id = $1 RETURNING id")
            .bind(question_id)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| DbError::Deletion(e))
        {
            Ok(id) => {
                // Commit the transaction
                tx.commit().await.map_err(|e| DbError::Access(e))?;
                Ok(id)
            },
            Err(e) => Err(e)
        }
    }

    async fn increment_question_likes(&self, question_id: EntityId) -> Result<(), DbError> {
        // Attempt to parse entity id
        let question_id: Uuid = question_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        // Ensure that both transactions occur by using a Transaction
        let mut tx = self.pool.begin().await.map_err(|e| DbError::Access(e))?;
        let likes = sqlx::query("SELECT likes FROM questions WHERE id = $1")
            .bind(question_id)
            .map(|row: PgRow| row.get::<i32, &str>("likes"))
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| DbError::NotFound(e))?;
        match sqlx::query("UPDATE questions SET likes = $1 WHERE id = $2")
            .bind(likes + 1)
            .bind(question_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DbError::Update(e))
        {
            Ok(_) => tx.commit().await.map_err(|e| DbError::Commit(e)),
            Err(e) => Err(e)
        }
    }
}

pub struct AnswerDaoImpl {
    pool: PgPool,
}

impl AnswerDao for AnswerDaoImpl {
    async fn create_answer(&self, new_answer: NewAnswer) -> Result<Uuid, DbError> {
        // First parse question_id
        let question_id: Uuid = Uuid::parse_str(new_answer.question_id.as_str()).map_err(|_| DbError::InvalidUuid("invalid uuid"))?;
        // Attempt to insert a new answer into the database
        sqlx::query("INSERT INTO answers (question_id, answer) VALUES ($1, $2) returning id")
            .bind(question_id)
            .bind(new_answer.answer)
            .map(|row| -> Uuid { row.get("id") })
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::Creation(e))
    }

    async fn get_answer(&self, answer_id: EntityId) -> Result<Answer, DbError> {
        // Parse answer id
        let answer_id: Uuid = answer_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        // attempt to read answer from database
        sqlx::query_as::<_, Answer>("SELECT * FROM answers WHERE id = $1")
            .bind(answer_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::Access(e))
    }

    async fn get_answers(&self, question_id: EntityId) -> Result<Vec<Answer>, DbError> {
        // Parse entity id first
        let question_id: Uuid = question_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        // Attempt to read all associated answers from database
        sqlx::query("SELECT * FROM answers WHERE question_id = $1")
            .bind(question_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::Access(e))?
            .into_iter()
            .map(|row| Answer::from_row(&row).map_err(|e| DbError::FromRow(e)))
            .collect::<Result<Vec<Answer>, DbError>>()
    }

    async fn delete_answer(&self, answer_id: EntityId) -> Result<Uuid, DbError> {
        // Parse entity id
        let answer_id: Uuid = answer_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        // Attempt to execute query
        match sqlx::query("DELETE * FROM answers WHERE id = $1")
            .bind(answer_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::Access(e))
        {
            Ok(_) => Ok(answer_id),
            Err(e) => Err(e)
        }
    }

    async fn get_all_answers(&self) -> Result<Vec<Answer>, DbError> {
        // Execute query
        sqlx::query_as::<_, Answer>("SELECT * FROM answers")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::Access(e))
    }

    async fn increment_answer_likes(&self, answer_id: EntityId) -> Result<(), DbError> {
        // Parse entity id
        let answer_id: Uuid = answer_id.try_into().map_err(|e| DbError::InvalidUuid(e))?;
        // Attempt to execute query, use a transaction
        let mut tx = self.pool.begin().await.map_err(|e| DbError::Access(e))?;
        let likes = sqlx::query("SELECT likes FROM answers WHERE id = $1")
            .bind(answer_id)
            .map(|row| row.get::<i32, &str>("id"))
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| DbError::NotFound(e))?;
        // Attempt to update database
        match sqlx::query("UPDATE answers SET likes = $1 WHERE id = $2")
            .bind(likes + 1)
            .bind(answer_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DbError::Update(e))
        {
            Ok(_) => tx.commit().await.map_err(|e| DbError::Commit(e)),
            Err(e) => Err(e)
        }

    }
}

