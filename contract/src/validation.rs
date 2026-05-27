#![allow(clippy::len_zero)]

use crate::error::LumentixError;
use soroban_sdk::{Address, String};

/// Validate that an address is not invalid
/// In Soroban, addresses are validated by the SDK, but we keep this for consistency
pub fn validate_address(_address: &Address) -> Result<(), LumentixError> {
    // Soroban SDK ensures addresses are valid
    // This function exists for future custom validation if needed
    Ok(())
}

/// Validate that an amount is positive (greater than 0)
pub fn validate_positive_amount(amount: i128) -> Result<(), LumentixError> {
    if amount <= 0 {
        return Err(LumentixError::InvalidAmount);
    }
    Ok(())
}

/// Validate that capacity is positive (greater than 0)
pub fn validate_positive_capacity(capacity: u32) -> Result<(), LumentixError> {
    if capacity == 0 {
        return Err(LumentixError::CapacityExceeded);
    }
    Ok(())
}

/// Validate that start time is before end time
pub fn validate_time_range(start_time: u64, end_time: u64) -> Result<(), LumentixError> {
    if start_time >= end_time {
        return Err(LumentixError::InvalidTimeRange);
    }
    Ok(())
}

/// Validate that a string is not empty
pub fn validate_string_not_empty(s: &String) -> Result<(), LumentixError> {
    if s.len() == 0 {
        return Err(LumentixError::EmptyString);
    }
    Ok(())
}

/// Validate that a VIP tier slot count is positive
pub fn validate_positive_slots(slots: u32) -> Result<(), LumentixError> {
    if slots == 0 {
        return Err(LumentixError::CapacityExceeded);
    }
    Ok(())
}

/// Validate that accessibility counts are valid
pub fn validate_accessibility_counts(available: u32, total: u32) -> Result<(), LumentixError> {
    if available > total {
        return Err(LumentixError::AccommodationUnavailable);
    }
    Ok(())
}

/// Validate that a seat identifier is valid
pub fn validate_seat_id(seat_id: &String) -> Result<(), LumentixError> {
    if seat_id.len() == 0 {
        return Err(LumentixError::InvalidSeatCategory);
    }
    Ok(())
}

/// Validate that a currency code is non-empty
pub fn validate_currency_code(code: &String) -> Result<(), LumentixError> {
    if code.len() == 0 {
        return Err(LumentixError::EmptyString);
    }
    if code.len() < 3 {
        return Err(LumentixError::UnsupportedCurrency);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_validate_positive_amount() {
        assert!(validate_positive_amount(100).is_ok());
        assert!(validate_positive_amount(1).is_ok());
        assert_eq!(
            validate_positive_amount(0),
            Err(LumentixError::InvalidAmount)
        );
        assert_eq!(
            validate_positive_amount(-1),
            Err(LumentixError::InvalidAmount)
        );
    }

    #[test]
    fn test_validate_positive_capacity() {
        assert!(validate_positive_capacity(100).is_ok());
        assert!(validate_positive_capacity(1).is_ok());
        assert_eq!(
            validate_positive_capacity(0),
            Err(LumentixError::CapacityExceeded)
        );
    }

    #[test]
    fn test_validate_time_range() {
        assert!(validate_time_range(100, 200).is_ok());
        assert!(validate_time_range(0, 1).is_ok());
        assert_eq!(
            validate_time_range(200, 100),
            Err(LumentixError::InvalidTimeRange)
        );
        assert_eq!(
            validate_time_range(100, 100),
            Err(LumentixError::InvalidTimeRange)
        );
    }

    #[test]
    fn test_validate_string_not_empty() {
        let env = Env::default();
        let valid_string = String::from_str(&env, "test");
        let empty_string = String::from_str(&env, "");

        assert!(validate_string_not_empty(&valid_string).is_ok());
        assert_eq!(
            validate_string_not_empty(&empty_string),
            Err(LumentixError::EmptyString)
        );
    }

    #[test]
    fn test_validate_positive_slots() {
        assert!(validate_positive_slots(1).is_ok());
        assert!(validate_positive_slots(100).is_ok());
        assert_eq!(
            validate_positive_slots(0),
            Err(LumentixError::CapacityExceeded)
        );
    }

    #[test]
    fn test_validate_accessibility_counts() {
        assert!(validate_accessibility_counts(5, 10).is_ok());
        assert!(validate_accessibility_counts(10, 10).is_ok());
        assert_eq!(
            validate_accessibility_counts(11, 10),
            Err(LumentixError::AccommodationUnavailable)
        );
    }

    #[test]
    fn test_validate_seat_id() {
        let env = Env::default();
        let valid = String::from_str(&env, "A-1-1");
        let empty = String::from_str(&env, "");

        assert!(validate_seat_id(&valid).is_ok());
        assert_eq!(
            validate_seat_id(&empty),
            Err(LumentixError::InvalidSeatCategory)
        );
    }

    #[test]
    fn test_validate_currency_code() {
        let env = Env::default();
        let valid = String::from_str(&env, "USD");
        let short = String::from_str(&env, "AB");
        let empty = String::from_str(&env, "");

        assert!(validate_currency_code(&valid).is_ok());
        assert_eq!(
            validate_currency_code(&short),
            Err(LumentixError::UnsupportedCurrency)
        );
        assert_eq!(
            validate_currency_code(&empty),
            Err(LumentixError::EmptyString)
        );
    }
}
