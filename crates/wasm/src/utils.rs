const API_URL: &str = std::env!("API_URL");

pub fn get_url(url: &str) -> String {
    format!("{}{}", API_URL, url)
}
