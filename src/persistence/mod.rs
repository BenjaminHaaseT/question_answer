//! Contains the trait needed for implementing a database access object as well
//! as implementations.

use sqlx::types::Uuid;
use crate::models::prelude::*;

/// The interface for any database access object that will interact with the the questions database.
pub trait QuestionDao {
    /// Creates a new question and inserts it into the database.
    ///
    /// # Parameters
    /// `new_question`: The content of the new question to be created and inserted into the database
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the question was created successfully a `Ok(Uuid)` will be returned
    /// where the `Uuid` represents the id of the newly created question, otherwise `Err(DbError)` will be returned.
    async fn create_question(new_question: NewQuestion) -> Result<Uuid, DbError>;

    /// Gets a question from the database if present.
    ///
    /// # Parameters
    /// `question_id` the `EntityId` of the question being queried
    ///
    ///# Returns
    /// A `Result<Question, DbError>`, a `Ok(Question)` if the query is successful, otherwise `Err(DbError)`.
    async fn get_question(question_id: EntityId) -> Result<Question, DbError>;

    /// Gets a `Vec` of all questions in the database
    ///
    /// # Returns
    /// A `Result<Vec<Question>>, DbError>`, in the success case `Ok(Vec<Question>)`, otherwise `Err(DbError)`.
    async fn get_questions() -> Result<Vec<Question>, DbError>;

    /// Deletes a question from the database.
    ///
    /// # Parameters
    /// `question_id` the `EntityId` of the `Question` to be deleted.
    ///
    /// # Returns
    /// A `Result<Uuid, DbError>`, if the question is successfully deleted then a `Ok(Uuid)` will be returned,
    /// otherwise an `Err(DbError)` is returned.
    async fn delete_question(question_id: EntityId) -> Result<Uuid, DbError>;

    /// Increments the number of likes associated with a particular question
    ///
    /// # Parameters
    /// `question_id`, the `EntityId` of the `Question` being queried
    ///
    /// # Returns
    /// A `Result<(), DbError>`, `Ok(())` in the successful case and `Err(DbError)` in the
    /// unsuccessful case.
    async fn increment_question_likes(question_id: EntityId) -> Result<(), DbError>;
}

/// The interface for any database access object that will interact with the answers database.
pub trait AnswerDao {
    async fn create_answer(question_id: EntityId, new_answer: NewAnswer) -> Result<Uuid, DbError>;

    async fn get_answer(answer_id: EntityId) -> Result<Answer, DbError>;

    async fn get_answers(question_id: EntityId) -> Result<Vec<Answer>, DbError>;

    async fn get_all_answers() -> Result<Vec<Answer>, DbError>;

    async fn delete_answer(answer_id: EntityId) -> Result<(), DbError>;

    async fn increment_answer_likes(answer_id: EntityId) -> Result<(), DbError>;

}

