pub fn create_message_whether(zip_code: &str) -> String {
    if zip_code.trim().is_empty() {
        return "Zip code is empty".to_string();
    }

    // Call an external API to get basic weather information
    let message = "is sunny today!";

    format!("The weather in your area (ZIP: {}) {}", zip_code, message)
}

#[cfg(test)]
mod tests {}
