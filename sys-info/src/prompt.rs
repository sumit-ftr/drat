pub fn fetch_prompt() -> String {
    let user = std::env::var("USER").unwrap();
    let hostname = std::env::var("HOSTNAME").unwrap();
    format!("{user}@{hostname}\n")
}
