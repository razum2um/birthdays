use crate::errors::AppError;
use chrono::{NaiveDate, Utc};
use std::char;

pub fn valid_username(username: String) -> Result<String, AppError> {
    if !username.is_empty() && username.chars().all(char::is_alphabetic) {
        Ok(username)
    } else {
        Err(AppError::InvalidUsername)
    }
}

pub fn valid_date(date: NaiveDate) -> Result<NaiveDate, AppError> {
    if (Utc::now().date_naive() - date).num_days() > 0 {
        Ok(date)
    } else {
        Err(AppError::InvalidBirthdate)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Days;

    #[test]
    fn test_valid_username() {
        assert!(valid_username("qwerty".to_string()).is_ok());
        assert!(valid_username("user123".to_string()).is_err());
        assert!(valid_username("".to_string()).is_err())
    }

    #[test]
    fn test_valid_date() {
        let today = Utc::now().date_naive();
        assert!(valid_date(today).is_err());

        let yesterday = today - Days::new(1);
        assert!(valid_date(yesterday).is_ok());

        let tomorrow = today + Days::new(1);
        assert!(valid_date(tomorrow).is_err())
    }
}
