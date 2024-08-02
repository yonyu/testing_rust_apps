use chrono::{Duration, Local};
use testing_rust_apps::Person;

#[test]
fn greeting_for_non_birthday() {
    let person = Person {
        name: "Alice".to_string(),
        date_of_birth: Local::now().naive_local().date() - Duration::days(1),
        zip_code: "12345".to_string(),
    };

    let greeting = person.create_greeting();
    assert!(greeting.contains("Hello, Alice!"));
    assert!(!greeting.contains("Happy birthday"));
    assert!(greeting.contains("12345"));
}