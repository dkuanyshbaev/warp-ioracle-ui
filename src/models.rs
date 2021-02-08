use serde_derive::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub email: String,
    pub question: String,
}

pub fn save(_db: SqlitePool, _question: Question, _answer: &str) -> Option<String> {
    let uuid = Uuid::new_v4();
    let uuid = uuid.to_string();

    // ---------------------------------------------------
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&db)
    //     .await
    //     .unwrap_or_else(|err| {
    //         println!("Test: {}", err);
    //         std::process::exit(1);
    //     });
    //
    // println!("{:?}", row);

    // ---------------------------------------------------
    // let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    // let id = sqlx::query!(
    //     r#"
    //     INSERT INTO todos ( description )
    //     VALUES ( ?1 )
    //     "#,
    //     description
    // )
    //     .execute(&mut conn)
    //     .await?
    //     .last_insert_rowid();
    //
    // Ok(id)
    // ---------------------------------------------------

    Some(uuid)
}
