use sqlx::postgres::{PgPoolOptions,PgPool};
use std::env;

pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
        Ok(pool)=>{
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err)=>{
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
        
    };
    pool
}

