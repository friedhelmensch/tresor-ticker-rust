extern crate chrono;
use crate::operations::error::Error;
use chrono::prelude::*;
use std::collections::HashMap;

pub fn get_menu_by_day(preformatted_menu: Vec<String>, day: Weekday) -> Result<Vec<String>, Error> {
    if day == Weekday::Sat || day == Weekday::Sun {
        return Err(Error::new(String::from("no menu on weekends")));
    }

    let weekly_dishes = extract_weekly_dishes(preformatted_menu.to_owned());
    let daily_dishes_by_weekday = extract_daily_dishes(preformatted_menu.to_owned());

    let dish_of_the_day = match day {
        Weekday::Mon => daily_dishes_by_weekday["Montag"].to_owned(),
        Weekday::Tue => daily_dishes_by_weekday["Dienstag"].to_owned(),
        Weekday::Wed => daily_dishes_by_weekday["Mittwoch"].to_owned(),
        Weekday::Thu => daily_dishes_by_weekday["Donnerstag"].to_owned(),
        Weekday::Fri => daily_dishes_by_weekday["Freitag"].to_owned(),
        _ => panic!["You are not supposed to be here"],
    };

    let mut dishes_of_the_day: Vec<String> = Vec::new();
    dishes_of_the_day.push(dish_of_the_day);
    for weekly_dish in weekly_dishes {
        dishes_of_the_day.push(weekly_dish.to_owned());
    }
    return Ok(dishes_of_the_day);
}

fn extract_weekly_dishes(preformatted_menu: Vec<String>) -> Vec<String> {
    let index_of_taeglich = preformatted_menu
        .iter()
        .position(|r| r == "Täglich")
        .unwrap();

    let weekly_dishes: Vec<String> = preformatted_menu.to_owned().split_off(index_of_taeglich);
    let weekly_dishes: Vec<String> = weekly_dishes
        .into_iter()
        .filter(|x| x != &"Täglich")
        .collect();

    return weekly_dishes;
}

fn extract_daily_dishes(preformatted_menu: Vec<String>) -> HashMap<String, String> {
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
                preformatted_menu[position_of_current_entry + 1].to_owned(),
            );
        }
    }

    return dishes_by_day;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_weekly_dishes_test() {
        let input = vec![
            String::from("Montag"),
            String::from("Rahmschnitzel mit Spätzle  €  8.00"),
            String::from("Dienstag"),
            String::from("Gyros mit Tzatziki und Pommes €  8,00"),
            String::from("Mittwoch"),
            String::from("Hähnchencurry mit Reis  €  8,00"),
            String::from("Donnerstag"),
            String::from("BBQ   €  9,90"),
            String::from("Freitag"),
            String::from("Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00"),
            String::from("Täglich"),
            String::from("Fritz Kuhn ist Bürgermeister"),
            String::from("Manuel Reiser hatte eine gute Idee"),
            String::from("Heute gibts Erdbeeren"),
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
            String::from("Montag"),
            String::from("Rahmschnitzel mit Spätzle  €  8.00"),
            String::from("Dienstag"),
            String::from("Gyros mit Tzatziki und Pommes €  8,00"),
            String::from("Mittwoch"),
            String::from("Hähnchencurry mit Reis  €  8,00"),
            String::from("Donnerstag"),
            String::from("BBQ   €  9,90"),
            String::from("Freitag"),
            String::from("Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00"),
            String::from("Täglich"),
            String::from("Salatteller mit Pangasiusfilet  € 7,50"),
            String::from("Chilli con Carne mit Brot € 7,00"),
            String::from("Tagliatelle mit frischen Pfifferlingen € 8,90"),
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
            String::from("Montag"),
            String::from("Rahmschnitzel mit Spätzle  €  8.00"),
            String::from("Dienstag"),
            String::from("Gyros mit Tzatziki und Pommes €  8,00"),
            String::from("Mittwoch"),
            String::from("Hähnchencurry mit Reis  €  8,00"),
            String::from("Donnerstag"),
            String::from("BBQ   €  9,90"),
            String::from("Freitag"),
            String::from("Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00"),
            String::from("Täglich"),
            String::from("Salatteller mit Pangasiusfilet  € 7,50"),
            String::from("Salatteller mit Pangasiusfilet  € 7,50"),
            String::from("Tagliatelle mit frischen Pfifferlingen € 8,90"),
        ];
        let result = get_menu_by_day(input, Weekday::Mon);
        println!("{:?}", result);

        let expected_result = vec![
            "Rahmschnitzel mit Spätzle  €  8.00".to_owned(),
            "Salatteller mit Pangasiusfilet  € 7,50".to_owned(),
            "Salatteller mit Pangasiusfilet  € 7,50".to_owned(),
            "Tagliatelle mit frischen Pfifferlingen € 8,90".to_owned(),
        ];
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn get_menu_by_days_sunday_test() {
        let input = vec![
            String::from("Montag"),
            String::from("Rahmschnitzel mit Spätzle  €  8.00"),
            String::from("Dienstag"),
            String::from("Gyros mit Tzatziki und Pommes €  8,00"),
            String::from("Mittwoch"),
            String::from("Hähnchencurry mit Reis  €  8,00"),
            String::from("Donnerstag"),
            String::from("BBQ   €  9,90"),
            String::from("Freitag"),
            String::from("Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00"),
            String::from("Täglich"),
            String::from("Salatteller mit Pangasiusfilet  € 7,50"),
            String::from("Salatteller mit Pangasiusfilet  € 7,50"),
            String::from("Tagliatelle mit frischen Pfifferlingen € 8,90"),
        ];
        let result = get_menu_by_day(input, Weekday::Sun);
        println!("{:?}", result);

        match result {
            Ok(_n) => panic!("expected error is missing"),
            Err(e) => assert_eq!(e.message, String::from("no menu on weekends")),
        };
    }
}
