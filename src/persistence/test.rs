use sqlx::test;
use sqlx::types::Uuid;
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
    let Err(DbError::Creation(e)) = question_res else {panic!("result should be a creation error")};
}

#[sqlx::test]
async fn get_question_should_fail(pool: PgPool) {
    let sample_id = Uuid::new_v4();
    let question_id = EntityId::new(sample_id.to_string());
    let question_dao = QuestionDaoImpl::new(pool);
    // Attempt to make query
    let get_res = question_dao.get_question(question_id).await;
    println!("{:?}", get_res);
    assert!(get_res.is_err());
}

#[sqlx::test]
async fn get_question_should_succeed(pool: PgPool) -> Result<(), DbError> {
    // First create a sample mock question
    let question_dao = QuestionDaoImpl::new(pool);
    let new_question = NewQuestion { title: String::from("Test Question"), question: String::from("Hello this question is a test") };
    let new_question_id = question_dao.create_question(new_question).await?;
    println!("new question uuid: {new_question_id}");
    // Create new entity id
    let question_id = EntityId::new(new_question_id.to_string());
    let get_res = question_dao.get_question(question_id).await;
    println!("{:?}", get_res);
    assert!(get_res.is_ok());
    Ok(())
}

#[sqlx::test]
async fn get_question_should_fail_with_invalid_uuid(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    let question_id = EntityId::new(String::from("invalid Uuid"));
    let get_res = question_dao.get_question(question_id).await;
    println!("{:?}", get_res);
    assert!(get_res.is_err());
    let Err(DbError::InvalidUuid(s)) = get_res else {panic!("Error should be a `InvalidUuid`")};
}

#[sqlx::test]
async fn get_questions_should_fail(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool.clone());
    pool.close().await;
    let get_res = question_dao.get_questions().await;
    println!("{:?}", get_res);
    assert!(get_res.is_err());
}

#[sqlx::test]
async fn get_questions_should_succeed_in_empty_state(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    let get_res = question_dao.get_questions().await;
    println!("{:?}", get_res);
    assert!(get_res.is_ok());
    assert_eq!(get_res.unwrap().len(), 0);
}

#[sqlx::test]
async fn get_questions_should_succeed_in_non_empty_state(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    let new_question1 = NewQuestion { title: String::from("Test Question1"), question: String::from("Hello this question is a test") };
    let new_question2 = NewQuestion { title: String::from("Test Question2"), question: String::from("Hello this question is a test") };
    let new_question3 = NewQuestion { title: String::from("Test Question3"), question: String::from("Hello this question is a test") };
    // Insert into database
    let new_question1_id = question_dao.create_question(new_question1).await.expect("question should be created successfully");
    let new_question2_id = question_dao.create_question(new_question2).await.expect("question should be created successfully");
    let new_question3_id = question_dao.create_question(new_question3).await.expect("question should be created successfully");
    // Attempt to get records from the database
    let get_res = question_dao.get_questions().await;
    println!("{:?}", get_res);
    assert!(get_res.is_ok());
    let questions = get_res.unwrap();
    // Ensure that we can find a question with each id that has been inserted into the database
    assert!(questions.iter().find(|q| q.id() == new_question1_id).is_some());
    assert!(questions.iter().find(|q| q.id() == new_question2_id).is_some());
    assert!(questions.iter().find(|q| q.id() == new_question3_id).is_some());
}

#[sqlx::test]
async fn delete_question_should_fail_with_not_found(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    let id = Uuid::new_v4();
    let question_id = EntityId::new(id.to_string());
    let del_res = question_dao.delete_question(question_id).await;
    println!("{:?}", del_res);
    assert!(del_res.is_err());
    let Err(DbError::NotFound(e)) = del_res else {panic!("error should be `Deletion`")};
}

#[sqlx::test]
async fn delete_question_should_succeed(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    // insert a question into the database
    let new_question = NewQuestion { title: String::from("Test Question1"), question: String::from("Hello this question is a test") };
    let new_question_id = question_dao.create_question(new_question)
        .await;
    println!("{:?}", new_question_id);
    assert!(new_question_id.is_ok());
    let new_question_id = EntityId::new(new_question_id.unwrap().to_string());
    let deleted_question_id = question_dao.delete_question(new_question_id).await;
    println!("{:?}", deleted_question_id);
    assert!(deleted_question_id.is_ok());
}

#[sqlx::test]
async fn increment_question_likes_should_fail_with_not_found(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    // insert new question into database
    let question_id = EntityId::new(Uuid::new_v4().to_string());
    let inc_res = question_dao.increment_question_likes(question_id).await;
    println!("{:?}", inc_res);
    assert!(inc_res.is_err());
    let Err(DbError::NotFound(e)) = inc_res else { panic!("Error should be `NotFound` variant") };
}

#[sqlx::test]
async fn increment_question_likes_should_succeed(pool: PgPool) {
    let question_dao = QuestionDaoImpl::new(pool);
    let new_question = NewQuestion { title: String::from("Test Question1"), question: String::from("Hello this question is a test") };
    let question_id = question_dao.create_question(new_question).await;
    println!("{:?}", question_id);
    assert!(question_id.is_ok());
    let question_id = EntityId::new(question_id.unwrap().to_string());
    let inc_res = question_dao.increment_question_likes(question_id).await;
    println!("{:?}", inc_res);
    assert!(inc_res.is_ok());
}