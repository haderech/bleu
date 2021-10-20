use regex::Regex;

use crate::error::error::ExpectedError;

pub fn hex_to_decimal(hex_str: &str) -> Result<String, ExpectedError> {
    let regex = Regex::new(r"^(0[xX])?[A-Fa-f0-9]+$").unwrap();
    match regex.is_match(hex_str) {
        true => {
            let prefix_removed = hex_str.trim_start_matches("0x");
            match primitive_types::U256::from_str_radix(prefix_removed, 16) {
                Ok(decimal_u256) => Ok(decimal_u256.to_string()),
                Err(err) => Err(ExpectedError::ParsingError(err.to_string()))
            }
        }
        false => Err(ExpectedError::InvalidError(format!("input value is not hex string! input={}", hex_str)))
    }
}

#[cfg(test)]
mod number {
    use crate::libs::number::hex_to_decimal;

    #[test]
    fn hex_to_decimal_test() {
        let decimal_str = hex_to_decimal("0x16345785d8a0000").unwrap();
        assert_eq!("100000000000000000", decimal_str);

        let decimal_str = hex_to_decimal("16345785d8a0000").unwrap();
        assert_eq!("100000000000000000", decimal_str);
    }

    #[test]
    fn hex_to_decimal_fail_test() {
        let result = hex_to_decimal("0x16345785d8a0000z");
        assert!(result.is_err());

        let result = hex_to_decimal("xx16345785d8a0000");
        assert!(result.is_err());
    }
}