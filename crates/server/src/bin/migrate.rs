use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file in server");

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("connect to database error");

    println!("running migration...");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    println!("migration completed!");

    Ok(())
}
