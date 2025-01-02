use time::{format_description, Date, Error, UtcOffset, Weekday};

include!(concat!(env!("OUT_DIR"), "/holiday_data.rs"));

/// Check if a given date is an off-day (holiday or weekend)
///
/// # Arguments
///
/// * `date` - Date string in YYYY-MM-DD format
///
/// # Returns
///
/// * `Ok((bool, Option<&'static str>))` - A tuple containing:
///   * boolean indicating if it's an off-day (holiday or weekend)
///   * Optional holiday name if it exists
/// * `Err` - If the date string is invalid
pub fn is_offday(date: &str) -> Result<(bool, Option<&'static str>), Error> {
    let format = format_description::parse("[year]-[month]-[day]")?;
    let date = Date::parse(date, &format)?;
    let year = date.year();
    let weekday = date.weekday();

    if let Some(year_data) = get_year_data(year) {
        if let Some((name, is_off_day)) = year_data.get(&date.format(&format).unwrap()) {
            return Ok((*is_off_day, Some(name)));
        }
    }

    // If not in holiday data, check if it's a weekend
    Ok((matches!(weekday, Weekday::Saturday | Weekday::Sunday), None))
}

/// Check if a given date is a workday
///
/// # Arguments
///
/// * `date` - Date string in YYYY-MM-DD format
///
/// # Returns
///
/// * `Ok(bool)` - true if it's a workday, false otherwise
/// * `Err` - If the date string is invalid
pub fn is_workday(date: &str) -> Result<bool, Error> {
    let (is_off, _) = is_offday(date)?;
    Ok(!is_off)
}

/// Check if current time (in UTC+8) is an off-day
///
/// # Returns
///
/// * `(bool, Option<&'static str>)` - A tuple containing:
///   * boolean indicating if it's an off-day (holiday or weekend)
///   * Optional holiday name if it exists
pub fn is_now_offday() -> (bool, Option<&'static str>) {
    let now = time::OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(8, 0, 0).unwrap());
    let date = now.date();
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    is_offday(&date.format(&format).unwrap()).unwrap()
}

/// Check if current time (in UTC+8) is a workday
///
/// # Returns
///
/// * `bool` - true if it's a workday, false otherwise
pub fn is_now_workday() -> bool {
    let (is_off, _) = is_now_offday();
    !is_off
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_offday() {
        // Test a known holiday
        let result = is_offday("2025-01-01").unwrap();
        assert!(result.0); // Should be an off-day
        assert_eq!(result.1.unwrap(), "元旦"); // Should be New Year's Day

        // Test a regular workday
        let result = is_offday("2025-01-02").unwrap(); // Thursday
        assert!(!result.0); // Should not be an off-day
        assert_eq!(result.1, None);

        // Test a weekend
        let result = is_offday("2025-01-04").unwrap(); // Saturday
        assert!(result.0); // Should be an off-day (weekend)
        assert_eq!(result.1, None);

        // Test a weekend marked as workday
        let result = is_offday("2025-09-28").unwrap(); // Sunday marked as workday
        assert!(!result.0); // Should not be an off-day
        assert_eq!(result.1.unwrap(), "国庆节、中秋节");
    }

    #[test]
    fn test_is_workday() {
        // Test a holiday
        assert!(!is_workday("2025-01-01").unwrap()); // New Year's Day

        // Test a normal weekday
        assert!(is_workday("2025-01-02").unwrap()); // Thursday

        // Test a weekend
        assert!(!is_workday("2025-01-04").unwrap()); // Saturday

        // Test a weekend marked as workday
        assert!(is_workday("2025-09-28").unwrap()); // Sunday marked as workday
    }

    #[test]
    fn test_invalid_date() {
        assert!(is_offday("invalid-date").is_err());
        assert!(is_workday("invalid-date").is_err());
    }
}
