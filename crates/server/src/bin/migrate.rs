use server::{
    db::{create_config, create_db},
    infrastructure,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    infrastructure::tracing::init_tracing();

    let config = create_config();

    let _ = create_db(&config).await;

    Ok(())
}
