mod message_whether;
mod message_birthday;
mod message_name;

pub struct Person {
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub zip_code: String,
}

impl Person {
    pub fn create_greeting(&self) -> String {
        let name_message = message_name::create_message_name(&self.name);
        let birthday_message = message_birthday::create_message_birthday(self.date_of_birth);
        let weather_message = message_whether::create_message_whether(&self.zip_code);

        format!("{} {} {}", name_message, birthday_message, weather_message)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn divide(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a as f32 / b as f32
}

pub fn check_even(num: i32) -> Result<(), String> {
    if num % 2 == 0 {
        Ok(())
    } else {
        Err("Number is not even".to_string())
    }
}

#[cfg(test)]
mod tests {}
