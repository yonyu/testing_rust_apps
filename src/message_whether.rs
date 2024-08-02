fn call_api(zip_code: &str) -> String {
    // Call an external API to get basic weather information
    "is sunny today!".to_string()
}

fn format_final_message(zip_code: &str, message: &str) -> String {
    format!("The weather in your area (ZIP: {}) {}", zip_code, message)
}

pub fn create_message_whether(zip_code: &str) -> String {
    if zip_code.trim().is_empty() {
        return "Zip code is empty".to_string();
    }

    // Call an external API to get basic weather information
    let message = "is sunny today!";

    format!("The weather in your area (ZIP: {}) {}", zip_code, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_message_blank_zip_code() {
        let zip_code = "";
        let result = create_message_whether(zip_code);
        assert_eq!(result, "Zip code is empty".to_string());
    }

    #[test]
    fn test_weather_message_valid_zip_code() {
        let zip_code = "12345";
        let weather_message = create_message_whether(zip_code);
        assert_eq!(weather_message, "The weather in your area (ZIP: 12345) is sunny today!".to_string());
    }
}
