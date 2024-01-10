//! Contains the structs that model the databases.

use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;

pub mod prelude {
    pub use super::*;
}

/// A new question received from a request.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewQuestion {
    /// The title of the new question
    pub title: String,
    /// The content of the new question
    pub question: String,
}

/// A question that has been successfully persisted in the database.
#[derive(Debug)]
pub struct Question {
    /// The unique id of the question
    id: Uuid,
    /// The title of the question
    title: String,
    /// The content of the question
    question: String,
    /// The number of likes the question has received
    likes: u32,
    /// The timestamp as a string the question was created
    created_at: String,
    // tags: Vec<Option<>>
}

/// A new answer to an associated question received from a request.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAnswer {
    /// The id of the question the new answer is responding to
    pub question_id: String,
    /// The content of the new answer
    pub answer: String,
}

/// An answer that has been successfully persisted in the database.
#[derive(Debug)]
pub struct Answer {
    /// The unique id of the answer
    id: Uuid,
    /// The unique id of the associated question
    question_id: Uuid,
    /// The content of the answer
    answer: String,
    /// The number of likes the answer has received
    likes: u32,
    /// The timestamp the answer was created at as a string
    created_at: String
}

/// A struct that acts as a wrapper for all entity ID's in the models module.
pub struct EntityId {
    id: String,
}

pub enum DbError {

}
