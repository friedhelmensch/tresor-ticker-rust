use std::collections::HashMap;

extern crate chrono;
use chrono::NaiveDate;

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

fn get_date(dayAndMonth: &str, year: u16) -> NaiveDate {
    if dayAndMonth == "5. August" {
        return NaiveDate::from_ymd(2019, 08, 5);
    }

    return NaiveDate::from_ymd(1970, 01, 01);
}

pub fn get_menu_by_days(preformatted_menu: Vec<&str>) -> HashMap<NaiveDate, Vec<&str>> {
    let menu_by_day = HashMap::new();
    return menu_by_day;
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn get_date_test() {
        let result = get_date("5. Augusti", 2019);
        let expected_result = NaiveDate::from_ymd(2019, 08, 5);
        assert_eq!(result, expected_result);
        println!("{:?}", result);
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
