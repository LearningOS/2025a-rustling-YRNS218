// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();

        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age_str = parts[1];
        if age_str.is_empty() {
            return Err(ParsePersonError::BadLen);
        }

        let age = age_str.parse().map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(Person::from_str(""), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        let p = Person::from_str("John, 30").unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_age() {
        assert_eq!(Person::from_str("John,"), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            Person::from_str("John, thirty"),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!(Person::from_str("John"), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(Person::from_str(", 30"), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert_eq!(Person::from_str(","), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            Person::from_str(", thirty"),
            Err(ParsePersonError::NoName)
        ));
    }
}