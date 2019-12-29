extern crate chrono;
use crate::operations::error::Error;
use chrono::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn get_menu_by_day(preformatted_menu: Vec<String>, day: Weekday) -> Result<Vec<String>, Error> {
    if day == Weekday::Sat || day == Weekday::Sun {
        return Err(Error::new(String::from("no menu on weekends")));
    }

    //do this to not break to public interface YET.
    let dishes: Vec<&str> = Vec::from_iter(preformatted_menu.iter().map(String::as_str));

    let weekly_dishes = extract_weekly_dishes(&dishes);
    let daily_dishes_by_weekday = extract_daily_dishes(&dishes);

    let dish_of_the_day = match day {
        Weekday::Mon => daily_dishes_by_weekday["Montag"],
        Weekday::Tue => daily_dishes_by_weekday["Dienstag"],
        Weekday::Wed => daily_dishes_by_weekday["Mittwoch"],
        Weekday::Thu => daily_dishes_by_weekday["Donnerstag"],
        Weekday::Fri => daily_dishes_by_weekday["Freitag"],
        _ => panic!["You are not supposed to be here"],
    };

    let dishes_of_the_day: Vec<&str> = [dish_of_the_day]
        .iter()
        .chain(&weekly_dishes)
        .map(|&x| x)
        .collect();

    //do this to not break to public interface YET.
    let dishes_of_the_day: Vec<String> =
        Vec::from_iter(dishes_of_the_day.into_iter().map(|x| x.to_owned()));

    return Ok(dishes_of_the_day);
}

fn extract_weekly_dishes<'a>(preformatted_menu: &Vec<&'a str>) -> Vec<&'a str> {
    let index_of_taeglich = preformatted_menu
        .iter()
        .position(|r| r == &"Täglich")
        .unwrap();

    let start_index = index_of_taeglich + 1;
    let end_index = preformatted_menu.len();

    let dishes: Vec<&str> = preformatted_menu[start_index..end_index]
        .iter()
        .map(|&x| x)
        .collect();

    return dishes;
}

fn extract_daily_dishes<'a>(preformatted_menu: &Vec<&'a str>) -> HashMap<&'a str, &'a str> {
    let dishes_by_day: HashMap<&'a str, &'a str> =
        preformatted_menu
            .iter()
            .fold(HashMap::new(), |mut hash_map, current| {
                if current == &"Montag"
                    || current == &"Dienstag"
                    || current == &"Mittwoch"
                    || current == &"Donnerstag"
                    || current == &"Freitag"
                {
                    let position_of_current_entry =
                        preformatted_menu.iter().position(|r| r == current).unwrap();
                    hash_map.insert(current, preformatted_menu[position_of_current_entry + 1]);
                }
                hash_map
            });

    return dishes_by_day;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_weekly_dishes_test() {
        let input = vec![
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
        let result = extract_weekly_dishes(&input);

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
        let result = extract_daily_dishes(&input);
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
