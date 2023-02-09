use chrono::{Datelike, NaiveDate};
use chronoutil::delta;

pub struct Person {
    birthday: NaiveDate,
}

impl Person {
    pub fn new(birthday: NaiveDate) -> Person {
        Person { birthday }
    }
}

pub trait BirthdayAwaiter {
    fn days_until_birthday(&self, on: NaiveDate) -> u16;
}

impl BirthdayAwaiter for Person {
    fn days_until_birthday(&self, on: NaiveDate) -> u16 {
        let birthday_this_year = delta::with_year(self.birthday, on.year());
        let until_birthday_this_year = (birthday_this_year - on).num_days();

        if until_birthday_this_year >= 0 {
            until_birthday_this_year as u16
        } else {
            let until_birthday_next_year = delta::with_year(self.birthday, on.year() + 1);
            (until_birthday_next_year - on).num_days() as u16
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn days_until(
        (year, month, day): (i32, u32, u32),
        (on_year, on_month, on_day): (i32, u32, u32),
    ) -> u16 {
        let birthday = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let on = NaiveDate::from_ymd_opt(on_year, on_month, on_day).unwrap();
        Person::new(birthday).days_until_birthday(on)
    }

    #[test]
    fn test_same_date() {
        assert_eq!(days_until((2023, 2, 5), (2023, 2, 5)), 0)
    }

    #[test]
    fn test_simple() {
        assert_eq!(days_until((1980, 2, 5), (2023, 1, 1)), 35);
        assert_eq!(days_until((1980, 2, 5), (2023, 2, 4)), 1);
        assert_eq!(days_until((1980, 2, 5), (2023, 2, 5)), 0);
        assert_eq!(days_until((1980, 2, 5), (2023, 2, 6)), 364);
        assert_eq!(days_until((1980, 2, 5), (2023, 12, 31)), 36)
    }

    #[test]
    fn test_leap_year_early() {
        assert_eq!(days_until((1981, 2, 5), (2024, 1, 1)), 35);
        assert_eq!(days_until((1981, 2, 5), (2024, 2, 5)), 0);
        assert_eq!(days_until((1981, 2, 5), (2024, 2, 28)), 343);
        assert_eq!(days_until((1981, 2, 5), (2024, 2, 29)), 342);
        assert_eq!(days_until((1981, 2, 5), (2024, 3, 1)), 341);
        assert_eq!(days_until((1981, 2, 5), (2024, 12, 31)), 36)
    }

    #[test]
    fn test_leap_year_later() {
        assert_eq!(days_until((1981, 3, 5), (2024, 1, 1)), 64);
        assert_eq!(days_until((1981, 3, 5), (2024, 2, 28)), 6);
        assert_eq!(days_until((1981, 3, 5), (2024, 2, 29)), 5);
        assert_eq!(days_until((1981, 3, 5), (2024, 3, 1)), 4);
        assert_eq!(days_until((1981, 3, 5), (2024, 3, 5)), 0);
        assert_eq!(days_until((1981, 3, 5), (2024, 12, 31)), 64)
    }

    #[test]
    fn test_born_on_feb_29_on_non_leap_year() {
        assert_eq!(days_until((1980, 2, 29), (2023, 1, 1)), 58);
        assert_eq!(days_until((1980, 2, 29), (2023, 2, 27)), 1);
        assert_eq!(days_until((1980, 2, 29), (2023, 2, 28)), 0);
        assert_eq!(days_until((1980, 2, 29), (2023, 3, 1)), 365);
        assert_eq!(days_until((1980, 2, 29), (2024, 12, 31)), 59);
    }

    #[test]
    fn test_born_on_feb_29_on_leap_year() {
        assert_eq!(days_until((1980, 2, 29), (2024, 1, 1)), 59);
        assert_eq!(days_until((1980, 2, 29), (2024, 2, 28)), 1);
        assert_eq!(days_until((1980, 2, 29), (2024, 2, 29)), 0);
        assert_eq!(days_until((1980, 2, 29), (2024, 3, 1)), 364);
        assert_eq!(days_until((1980, 2, 29), (2024, 12, 31)), 59);
    }
}
