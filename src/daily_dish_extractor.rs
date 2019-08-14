use std::collections::HashMap;

extern crate chrono;
use chrono::prelude::*;
use chrono::{Duration, NaiveDate};

fn extract_weekly_dishes(preformatted_menu: Vec<&str>) -> Vec<&str> {
    let index_of_taeglich = preformatted_menu
        .iter()
        .position(|&r| r == "Täglich")
        .unwrap();

    let weekly_dishes: Vec<&str> = preformatted_menu.to_owned().split_off(index_of_taeglich);
    let weekly_dishes: Vec<&str> = weekly_dishes
        .into_iter()
        .filter(|x| x != &"Täglich")
        .collect();

    return weekly_dishes;
}

fn extract_daily_dishes(preformatted_menu: Vec<&str>) -> HashMap<String, String> {
    let mut dishes_by_day = HashMap::new();
    for entry in &preformatted_menu {
        if entry == &"Montag"
            || entry == &"Dienstag"
            || entry == &"Mittwoch"
            || entry == &"Donnerstag"
            || entry == &"Freitag"
        {
            let position_of_current_entry =
                preformatted_menu.iter().position(|r| r == entry).unwrap();

            dishes_by_day.insert(
                entry.to_string(),
                preformatted_menu[position_of_current_entry + 1].to_string(),
            );
        }
    }

    return dishes_by_day;
}

fn get_date(day_and_month: &str, year: i32) -> NaiveDate {
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

pub fn get_menu_by_days(preformatted_menu: Vec<&str>) -> HashMap<NaiveDate, Vec<&str>> {
    let menu_by_day = HashMap::new();

    let weekly_dishes = extract_weekly_dishes(preformatted_menu.to_owned());
    let daily_dishes = extract_daily_dishes(preformatted_menu.to_owned());
    let start_and_end = extract_start_and_end_string(preformatted_menu[0]);

    return menu_by_day;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_start_and_end_string_test() {
        let result =
            extract_start_and_end_string("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr");
        let start = result[0];
        let end = result[1];
        assert_eq!(start, "5. August");
        assert_eq!(end, "9. August");
    }
    #[test]
    fn extract_weekly_dishes_test() {
        let input = vec![
            "Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr",
            "Montag",
            "Rahmschnitzel mit Spätzle  €  8.00",
            "Dienstag",
            "Gyros mit Tzatziki und Pommes €  8,00",
            "Mittwoch",
            "Hähnchencurry mit Reis  €  8,00",
            "Donnerstag",
            "BBQ   €  9,90",
            "Freitag",
            "Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00",
            "Täglich",
            "Salatteller mit Pangasiusfilet  € 7,50",
            "Chilli con Carne mit Brot € 7,00",
            "Tagliatelle mit frischen Pfifferlingen € 8,90",
        ];
        let result = extract_weekly_dishes(input);
        //println!("{:?}", result);
        assert!(result.contains(&"Salatteller mit Pangasiusfilet  € 7,50"));
        assert!(result.contains(&"Chilli con Carne mit Brot € 7,00"));
        assert!(result.contains(&"Salatteller mit Pangasiusfilet  € 7,50"));
        assert!(!result.contains(&"Täglich"));
    }
    #[test]
    fn extract_daily_dishes_test() {
        let input = vec![
            "Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr",
            "Montag",
            "Rahmschnitzel mit Spätzle  €  8.00",
            "Dienstag",
            "Gyros mit Tzatziki und Pommes €  8,00",
            "Mittwoch",
            "Hähnchencurry mit Reis  €  8,00",
            "Donnerstag",
            "BBQ   €  9,90",
            "Freitag",
            "Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00",
            "Täglich",
            "Salatteller mit Pangasiusfilet  € 7,50",
            "Chilli con Carne mit Brot € 7,00",
            "Tagliatelle mit frischen Pfifferlingen € 8,90",
        ];
        let result = extract_daily_dishes(input);
        println!("{:?}", result);
        assert!(result["Montag"] == "Rahmschnitzel mit Spätzle  €  8.00");
        assert!(result["Dienstag"] == "Gyros mit Tzatziki und Pommes €  8,00");
        assert!(result["Mittwoch"] == "Hähnchencurry mit Reis  €  8,00");
        assert!(result["Donnerstag"] == "BBQ   €  9,90");
        assert!(result["Freitag"] == "Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00");
    }
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
    /*
    #[test]
    fn get_menu_by_days_test() {
        let input = vec![
            "Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr",
            "Montag",
            "Rahmschnitzel mit Spätzle  €  8.00",
            "Dienstag",
            "Gyros mit Tzatziki und Pommes €  8,00",
            "Mittwoch",
            "Hähnchencurry mit Reis  €  8,00",
            "Donnerstag",
            "BBQ   €  9,90",
            "Freitag",
            "Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00",
            "Täglich",
            "Salatteller mit Pangasiusfilet  € 7,50",
            "Salatteller mit Pangasiusfilet  € 7,50",
            "Tagliatelle mit frischen Pfifferlingen € 8,90",
        ];
        let result = get_menu_by_days(input);
        //println!("{:?}", result);
        let monday = NaiveDate::from_ymd(2019, 08, 5);

        let result_on_monday = &result[&monday];

        assert!(result_on_monday.contains(&"Rahmschnitzel mit Spätzle  €  8.00"));
        assert!(result_on_monday.contains(&"Salatteller mit Pangasiusfilet  € 7,50"));
        assert!(result_on_monday.contains(&"Salatteller mit Pangasiusfilet  € 7,50"));
        assert!(result_on_monday.contains(&"Tagliatelle mit frischen Pfifferlingen € 8,90"));
    }
    */
}
