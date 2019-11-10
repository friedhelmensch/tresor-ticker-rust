use chrono::prelude::*;
use time::Duration;

pub fn get_date_of_monday(date_as_string: String, year: i32) -> NaiveDate {
    let start_and_end_date = extract_start_and_end_string(date_as_string);
    return get_date(start_and_end_date[0].clone(), year);
}

fn extract_start_and_end_string(line: String) -> Vec<String> {
    let split_at_von: Vec<&str> = line.split_terminator("von").collect();

    let start_and_end = split_at_von[0];
    let split_start_and_end: Vec<&str> = start_and_end.split_terminator("bis").collect();

    let start = split_start_and_end[0];
    let start: Vec<&str> = start.split_terminator("Vom").collect();
    let start = start[1].trim().to_string();

    let end = split_start_and_end[1].trim().to_string();

    return vec![start, end];
}

fn get_date(day_and_month: String, year: i32) -> NaiveDate {
    let day_and_month: Vec<&str> = day_and_month
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

pub fn get_date_of_monday_in_the_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    match weekday {
        Weekday::Mon => return date,
        Weekday::Tue => return date - Duration::days(1),
        Weekday::Wed => return date - Duration::days(2),
        Weekday::Thu => return date - Duration::days(3),
        Weekday::Fri => return date - Duration::days(4),
        Weekday::Sat => return date - Duration::days(5),
        Weekday::Sun => return date - Duration::days(6),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_date_of_monday_test() {
        let date_as_string =
            String::from("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr");
        let expected_date = NaiveDate::from_ymd(2019, 8, 5);
        let result = get_date_of_monday(date_as_string, 2019);
        assert_eq!(result, expected_date);
    }

    #[test]
    fn get_date_of_monday_overlapping_months_test() {
        let date_as_string = String::from("Vom 29. Juli bis 2. August von 11.30 Uhr bis 14.00 Uhr");
        let expected_date = NaiveDate::from_ymd(2019, 7, 29);
        let result = get_date_of_monday(date_as_string, 2019);
        assert_eq!(result, expected_date);
    }
}
