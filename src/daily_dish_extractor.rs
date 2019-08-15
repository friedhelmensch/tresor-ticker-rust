extern crate chrono;
use chrono::prelude::*;
use std::collections::HashMap;

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

pub fn get_menu_by_day(preformatted_menu: Vec<&str>, day: Weekday) -> Vec<String> {
    let weekly_dishes = extract_weekly_dishes(preformatted_menu.to_owned());
    let daily_dishes_by_weekday = extract_daily_dishes(preformatted_menu.to_owned());

    let dish_of_the_day;
    match day {
        Weekday::Mon => dish_of_the_day = daily_dishes_by_weekday["Montag"].to_owned(),
        Weekday::Tue => dish_of_the_day = daily_dishes_by_weekday["Dienstag"].to_owned(),
        Weekday::Wed => dish_of_the_day = daily_dishes_by_weekday["Mittwoch"].to_owned(),
        Weekday::Thu => dish_of_the_day = daily_dishes_by_weekday["Donnerstag"].to_owned(),
        Weekday::Fri => dish_of_the_day = daily_dishes_by_weekday["Freitag"].to_owned(),
        Weekday::Sat => dish_of_the_day = daily_dishes_by_weekday["Samstag"].to_owned(),
        Weekday::Sun => dish_of_the_day = daily_dishes_by_weekday["Sonntag"].to_owned(),
    }

    let mut dishes_of_the_day: Vec<String> = Vec::new();
    dishes_of_the_day.push(dish_of_the_day);
    for weekly_dish in weekly_dishes {
        dishes_of_the_day.push(weekly_dish.to_owned());
    }
    return dishes_of_the_day;
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
            "Fritz Kuhn ist Bürgermeister",
            "Manuel Reiser hatte eine gute Idee",
            "Heute gibts Erdbeeren",
        ];
        let result = extract_weekly_dishes(input);

        let expected_result = vec![
            "Fritz Kuhn ist Bürgermeister",
            "Manuel Reiser hatte eine gute Idee",
            "Heute gibts Erdbeeren",
        ];
        assert_eq!(result, expected_result);
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
        let result = get_menu_by_day(input, Weekday::Mon);
        println!("{:?}", result);

        let expected_result = vec![
            "Rahmschnitzel mit Spätzle  €  8.00".to_owned(),
            "Salatteller mit Pangasiusfilet  € 7,50".to_owned(),
            "Salatteller mit Pangasiusfilet  € 7,50".to_owned(),
            "Tagliatelle mit frischen Pfifferlingen € 8,90".to_owned(),
        ];
        assert_eq!(result, expected_result);
    }
}
