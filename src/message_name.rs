pub fn create_message_name(name: &str) -> String {
    if name.trim().is_empty() {
        return "Name cannot be blank".to_string();
    }

    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {}