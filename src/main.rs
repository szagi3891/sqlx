use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct EnvConfig {
    database_url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodoModel {
    id: i64,
    description: String,
    done: bool,
    author: i32,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let config = match envy::from_env::<EnvConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    println!("ddd {config:?}");

    let sqlx_pool = sqlx::PgPool::connect(config.database_url.as_str()).await.unwrap();

    println!("nawiązano połączenie z bazą");

    sqlx::migrate!("./migrations")
        .run(&sqlx_pool)
        .await.unwrap();

    println!("wykonano migracę 2");

    let nowy_model = sqlx::query_as!(TodoModel, 
        r#"
            INSERT INTO
                todos (description, done, author)
            VALUES
                ($1, $2, $3)
            RETURNING
                *
        "#,
        "dsadsasa",
        false,
        555
    ).fetch_one(&sqlx_pool).await.unwrap();


    println!("wstawiono model {nowy_model:#?}");

    // todo!()
}
