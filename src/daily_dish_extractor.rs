extern crate chrono;
use chrono::prelude::*;
use std::collections::HashMap;

macro_rules! string_hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key.to_string(), $val); )*
         map
    }}
}

fn extract_weekly_dishes(preformatted_menu: &Vec<String>) -> Vec<String> {
    return preformatted_menu
        .into_iter()
        .skip_while(|x| x != &"Täglich")
        .skip(1)
        .map(|x| x.to_owned())
        .collect();
}

fn create_weekday_map() -> HashMap<String, Weekday> {
    return string_hashmap![
        "Montag" => Weekday::Mon,
        "Dienstag" => Weekday::Tue,
        "Mittwoch" => Weekday::Wed,
        "Donnerstag" => Weekday::Thu,
        "Freitag" => Weekday::Fri,
        "Samstag" => Weekday::Sat,
        "Sonntag" => Weekday::Sun
    ];
}

fn extract_daily_dishes(preformatted_menu: &Vec<String>) -> HashMap<Weekday, String> {
    let weekday_map = create_weekday_map();
    return preformatted_menu
        .iter()
        .zip(preformatted_menu.iter().skip(1))
        .filter(|x| weekday_map.contains_key(x.0))
        .map(|x| (weekday_map[x.0], x.1.clone()))
        .collect::<HashMap<Weekday, String>>();
}

pub fn get_menu_by_day(preformatted_menu: Vec<String>, day: &Weekday) -> Vec<String> {
    return vec![
        vec![(&extract_daily_dishes(&preformatted_menu)[day]).to_owned()],
        extract_weekly_dishes(&preformatted_menu)
    ].concat();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_weekly_dishes_test() {
        let input = vec![
            String::from("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr"),
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
            String::from("HAAAAANNNNNNNNNNEESSSS hatte eine gute Idee"),
            String::from("Heute gibts Erdbeeren"),
        ];
        let result = extract_weekly_dishes(&input);

        let expected_result = vec![
            "Fritz Kuhn ist Bürgermeister",
            "HAAAAANNNNNNNNNNEESSSS hatte eine gute Idee",
            "Heute gibts Erdbeeren",
        ];
        assert_eq!(result, expected_result);
    }
    #[test]
    fn extract_daily_dishes_test() {
        let input = vec![
            String::from("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr"),
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
        let result = extract_daily_dishes(&input);
        println!("{:?}", result);
        assert!(result[&Weekday::Mon] == "Rahmschnitzel mit Spätzle  €  8.00");
        assert!(result[&Weekday::Tue] == "Gyros mit Tzatziki und Pommes €  8,00");
        assert!(result[&Weekday::Wed] == "Hähnchencurry mit Reis  €  8,00");
        assert!(result[&Weekday::Thu] == "BBQ   €  9,90");
        assert!(result[&Weekday::Fri] == "Tintenfischtulpen mit Rosmarinkartoffeln  €  8,00");
    }
    #[test]
    fn get_menu_by_days_test() {
        let input = vec![
            String::from("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr"),
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
        let result = get_menu_by_day(input, &Weekday::Mon);
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
