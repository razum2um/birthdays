use std::fmt::Display;
use std::string::ToString;

pub fn birthday_message<T: ToString + Display>(username: &T, days: u16) -> String {
    match days {
        0 => format!("Hello, {username}! Happy birthday!"),
        _ => format!("Hello, {username}! Your birthday is in {days} day(s)"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(
            birthday_message(&"john".to_string(), 0),
            "Hello, john! Happy birthday!"
        );
    }

    #[test]
    fn test_others() {
        assert_eq!(
            birthday_message(&"jack".to_string(), 2),
            "Hello, jack! Your birthday is in 2 day(s)"
        );
    }
}
