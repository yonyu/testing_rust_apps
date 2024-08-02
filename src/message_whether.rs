pub trait WeatherApi {
    fn get_weather(&self, zip_code: &str) -> String;
}

pub struct RealWeatherApi;

impl WeatherApi for RealWeatherApi {
    fn get_weather(&self, _zip_code: &str) -> String {
        // Original logic to call an external API to get basic weather information
        "is sunny today!".to_string()
    }
}

fn format_final_message(zip_code: &str, message: &str) -> String {
    format!("The weather in your area (ZIP: {}) {}", zip_code, message)
}

pub fn create_message_whether(api: &dyn WeatherApi, zip_code: &str) -> String {
    if zip_code.trim().is_empty() {
        panic!("Zip code is empty");
        // return "Zip code is empty".to_string();
    }

    // Call an external API to get basic weather information
    let message = api.get_weather(zip_code);

    format_final_message(zip_code, &message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Zip code is empty")]
    fn test_weather_message_blank_zip_code() {
        let zip_code = "";
        let api = RealWeatherApi;
        create_message_whether(&api, zip_code);
    }

    #[test]
    fn test_weather_message_valid_zip_code() {
        let zip_code = "12345";
        let api = RealWeatherApi;
        let weather_message = create_message_whether(&api, zip_code);
        assert_eq!(weather_message, "The weather in your area (ZIP: 12345) is sunny today!".to_string());
    }
}
