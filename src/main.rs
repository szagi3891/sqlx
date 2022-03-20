use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct EnvConfig {
    database_url: String
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

    println!("wykonano migracę");


}
