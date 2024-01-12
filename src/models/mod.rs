//! Contains the structs that model the databases.

use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
// use sqlx::uuid
use sqlx::error::Error;
use chrono::{DateTime, Utc};



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
#[derive(Debug, Serialize, FromRow)]
pub struct Question {
    /// The unique id of the question
    id: Uuid,
    /// The title of the question
    title: String,
    /// The content of the question
    question: String,
    /// The number of likes the question has received
    likes: i32,
    /// The timestamp as a string the question was created
    created_at: DateTime<Utc>,
    // tags: Vec<Option<>>
}

impl Question {
    pub fn new(id: Uuid, title: String, question: String, likes: i32, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            question,
            likes,
            created_at
        }
    }
    pub fn builder() -> QuestionBuilder {
        QuestionBuilder::new()
    }
}

pub struct QuestionBuilder {
    id: Option<Uuid>,
    title: Option<String>,
    question: Option<String>,
    likes: Option<i32>,
    created_at: Option<DateTime<Utc>>,
}

impl QuestionBuilder {
    fn new() -> Self {
        QuestionBuilder {
            id: None,
            title: None,
            question: None,
            likes: None,
            created_at: None,
        }
    }
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
#[derive(Debug, Serialize, FromRow)]
pub struct Answer {
    /// The unique id of the answer
    id: Uuid,
    /// The unique id of the associated question
    question_id: Uuid,
    /// The content of the answer
    answer: String,
    /// The number of likes the answer has received
    likes: i32,
    /// The timestamp the answer was created at as a string
    created_at: DateTime<Utc>
}

/// A struct that acts as a wrapper for all entity ID's in the models module.
pub struct EntityId {
    id: String,
}

impl TryInto<Uuid> for EntityId {
    type Error = &'static str;
    fn try_into(self) -> Result<Uuid, Self::Error> {
        Uuid::parse_str(self.id.as_str()).map_err(|_| "unable to parse as uuid")
    }
}



pub enum DbError {
    Creation(Error),
    NotFound(Error),
    InvalidUuid(&'static str),
    Access(Error),
    FromRow(Error),
    Deletion(Error),
    Update(Error),
    Commit(Error),
}
