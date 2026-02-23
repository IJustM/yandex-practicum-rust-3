use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port_http: u16,
    #[allow(dead_code)]
    pub port_grpc: u16,
    pub jwt_secret: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
        let port_http = std::env::var("PORT_HTTP")
            .unwrap_or("8080".into())
            .parse()?;
        let port_grpc = std::env::var("PORT_GRPC")
            .unwrap_or("8080".into())
            .parse()?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or("*".into());

        Ok(Self {
            database_url,
            host,
            port_http,
            port_grpc,
            jwt_secret,
            cors_origin,
        })
    }
}
