use chrono::{Datelike, Local, NaiveDate};

pub fn create_message_birthday(date_of_birth: NaiveDate) -> String {
    let today = Local::now().naive_local().date();

    if date_of_birth.month() == today.month() && date_of_birth.day() == today.day() {
        "Happy Birthday!".to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_message_today() {
        let today = Local::now().naive_local().date();
        let birthday_message = create_message_birthday(today);
        assert_eq!(birthday_message, "Happy Birthday!".to_string());
    }

    #[test]
    fn test_birthday_message_not_today() {
        let yesterday = Local::now().naive_local().date()-chrono::Duration::days(1);
        let birthday_message = create_message_birthday(yesterday);
        assert_eq!(birthday_message, "".to_string());
    }
}