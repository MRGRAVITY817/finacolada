use chrono::NaiveDate;

pub fn parse_to_naive_date(str_date: &str) -> anyhow::Result<NaiveDate> {
    if str_date.len().ne(&8) {
        return Err(anyhow::Error::msg(
            "Input date should be 8-digits long, in YYYYMMDD format.",
        ));
    }
    NaiveDate::from_ymd_opt(
        str_date[0..4].parse::<i32>()?,
        str_date[4..6].parse::<u32>()?,
        str_date[6..8].parse::<u32>()?,
    )
    .ok_or(anyhow::Error::msg("Cannot parse to date."))
}

#[cfg(test)]
mod test {
    use super::parse_to_naive_date;

    #[test]
    fn given_date_should_be_equal_to_8_digits() {
        // Arrange
        let too_long = "202301260";
        let too_short = "2023012";
        let just_right = "20230126";
        // Act & Assert
        assert!(parse_to_naive_date(too_long).is_err());
        assert!(parse_to_naive_date(too_short).is_err());
        assert!(parse_to_naive_date(just_right).is_ok());
    }

    #[test]
    fn given_date_should_be_valid_date() {
        // Arrange
        let wrong_date = "20231322";
        let correct_date = "19940817";
        // Act & Assert
        assert!(parse_to_naive_date(wrong_date).is_err());
        assert!(parse_to_naive_date(correct_date).is_ok());
    }
}
