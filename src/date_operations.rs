extern crate chrono;
use chrono::prelude::*;
use chrono::{Duration, NaiveDate};

fn extract_start_and_end_string(line: &str) -> Vec<&str> {
    let split_at_von: Vec<&str> = line.split_terminator("von").collect();

    let start_and_end = split_at_von[0];
    let split_start_and_end: Vec<&str> = start_and_end.split_terminator("bis").collect();

    let start = split_start_and_end[0];
    let start: Vec<&str> = start.split_terminator("Vom").collect();
    let start = start[1].trim();

    let end = split_start_and_end[1].trim();

    return vec![start, end];
}

fn get_date(day_and_month: &str, year: i32) -> NaiveDate {
    let day_and_month = day_and_month
        .split_terminator(".")
        .map(|x| x.trim())
        .collect();

    let day: u32 = day_and_month[0].parse().unwrap();
    let raw_month = day_and_month[1];

    let month: u32;
    match raw_month {
        "Januar" => month = 1,
        "Februar" => month = 2,
        "März" => month = 3,
        "April" => month = 4,
        "Mai" => month = 5,
        "Juni" => month = 6,
        "Juli" => month = 7,
        "August" => month = 8,
        "September" => month = 9,
        "Oktober" => month = 10,
        "November" => month = 11,
        "Dezember" => month = 12,
        _ => month = 0,
    }

    return NaiveDate::from_ymd(year, month, day);
}

fn get_dates_by_week_day(start: &str, end: &str, year: i32) -> HashMap<Weekday, NaiveDate> {
    let start = get_date(start, year);
    let end = get_date(end, year);

    let days_in_between = end.signed_duration_since(start).num_days() - 1;

    let mut dates_by_week_day = HashMap::new();
    dates_by_week_day.insert(start.weekday(), start);
    for n in 1..=days_in_between {
        let current_date = start + Duration::days(n);
        dates_by_week_day.insert(current_date.weekday(), current_date);
    }
    dates_by_week_day.insert(end.weekday(), end);
    return dates_by_week_day;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_date_5th_august_test() {
        let result = get_date("5. August", 2019);
        let expected_result = NaiveDate::from_ymd(2019, 08, 5);
        assert_eq!(result, expected_result);
    }
    #[test]
    fn get_date_1st_january_test() {
        let result = get_date("1. Januar", 2019);
        let expected_result = NaiveDate::from_ymd(2019, 1, 1);
        assert_eq!(result, expected_result);
    }
    #[test]
    fn get_date_31_december_test() {
        let result = get_date("31.Dezember", 2019);
        let expected_result = NaiveDate::from_ymd(2019, 12, 31);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn get_dates_by_week_day_test() {
        let result = get_dates_by_week_day("5. August", "9. August", 2019);
        let expected_date_on_thursday = NaiveDate::from_ymd(2019, 08, 08);
        let date_on_thursday = result[&Weekday::Thu];
        assert_eq!(expected_date_on_thursday, date_on_thursday);
    }


    #[test]
    fn extract_start_and_end_string_test() {
        let result =
            extract_start_and_end_string("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr");
        let start = result[0];
        let end = result[1];
        assert_eq!(start, "5. August");
        assert_eq!(end, "9. August");
    }
}