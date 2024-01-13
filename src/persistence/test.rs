use sqlx::test;
use crate::models::prelude::*;
use super::prelude::*;

#[sqlx::test]
async fn create_question_should_work(pool: PgPool) -> Result<(), DbError> {
    let question_dao = QuestionDaoImpl::new(pool);
    let new_question = NewQuestion { title: String::from("Test Question"), question: String::from("Hello this question is a test") };
    let question_res = question_dao.create_question(new_question).await;
    println!("{:?}", question_res);
    assert!(question_res.is_ok());
    Ok(())
}

#[sqlx::test]
async fn create_question_should_fail_with_creation_error(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool.clone());
    pool.close().await;
    let new_question = NewQuestion { title: String::from("Test Question"), question: String::from("Hello this question is a test") };
    let question_res = question_dao.create_question(new_question).await;
    println!("{:?}", question_res);
    assert!(question_res.is_err());
}