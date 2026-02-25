fn main() {
    println!("cargo:rerun-if-changed=.env");

    dotenvy::dotenv().ok();

    let api_url = std::env::var("API_URL").unwrap_or("http://127.0.0.1:8080".to_string());

    println!("cargo:rustc-env=API_URL={}", api_url);
}
