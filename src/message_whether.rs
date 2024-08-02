pub fn create_message_whether(zip_code: &str) -> String {
    if zip_code.trim().is_empty() {
        return "Zip code is empty".to_string();
    }
    let message = "is sunny today!".to_string();
    format!("The weather in your area (ZIP: {}) {}", zip_code, message)
}

#[cfg(test)]
mod tests {}
