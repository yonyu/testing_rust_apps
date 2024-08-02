pub fn create_message_name(name: &str) -> String {
    if name.trim().is_empty() {
        return "Name cannot be blank".to_string();
    }

    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Name cannot be blank")]
    fn test_blank_name() {
        let result = create_message_name("");
        assert_eq!(result, "Hello, !".to_string());
    }

    #[test]
    //#[ignore = "reason"]
    fn test_valid_name() {
        let name = "Alice";
        let result = create_message_name(name);
        assert_eq!(result, "Hello, Alice!".to_string());
    }
}