mod operations;

use crate::operations::error::Error;
use crate::operations::*;
use chrono::prelude::*;

pub fn get_dishes_of_the_day(raw_text: String) -> Result<Vec<String>, Error> {
    return get_dishes_of_the_day_by_day(raw_text, Utc::now().date().naive_utc());
}

fn get_dishes_of_the_day_by_day(raw_text: String, date: NaiveDate) -> Result<Vec<String>, Error> {
    let preformatted_menu = preformatting::pre_format_text(raw_text);
    let preformat_result = preformatting::split_date_and_dishes(preformatted_menu);

    let menu;
    match preformat_result {
        Ok(m) => menu = m,
        Err(error) => return Err(error),
    }

    let date_of_monday_of_menu = dates::get_date_of_monday(menu.date, date.year());
    let date_of_monday_of_this_week = dates::get_date_of_monday_in_the_week(date);

    if date_of_monday_of_menu != date_of_monday_of_this_week {
        return Err(Error::new(String::from(
            "Menu ist not up to date. Check again later.",
        )));
    }

    let dishes_of_the_day = extraction::get_menu_by_day(menu.dishes, date.weekday());

    match dishes_of_the_day {
        Ok(dishes) => Ok(dishes),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dishes_of_the_day_outdated_test() {
        let input = String::from(" \r\n \r\n \r\n \r\nWochenkarte \r\n \r\n \r\nVom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr \r\n \r\nMontag \r\n \r\nRahmschnitzel mit Spätzle  €  8.00 \r\n \r\nDienstag \r\n \r\nGyros mit Tzatziki und Pommes €  8,00 \r\n \r\nMittwoch \r\n \r\nHähnchencurry mit Reis  €  8,00 \r\n \r\nDonnerstag \r\n \r\n BBQ   €  9,90  \r\n \r\nFreitag  \r\n \r\nTintenfischtulpen mit Rosmarinkartoffeln  €  8,00   \r\n \r\n            \r\nTäglich \r\n \r\nSalatteller mit Pangasiusfilet  € 7,50 \r\nChilli con Carne mit Brot € 7,00 \r\nTagliatelle mit frischen Pfifferlingen € 8,90 \r\n----------------Page (0) Break----------------\r\n");
        let result = get_dishes_of_the_day_by_day(input, Utc::now().date().naive_utc());
        match result {
            Ok(_n) => panic!("expected an error for unmatching dates but got Ok."),
            Err(e) => assert_eq!(
                e.message,
                String::from("Menu ist not up to date. Check again later.")
            ),
        };
    }

    #[test]
    fn get_dishes_of_the_day_invalid_test() {
        let input = String::from("Invalid");
        let result = get_dishes_of_the_day_by_day(input, Utc::now().date().naive_utc());
        assert!(result.is_err(), "Invalid input should result in an error");
    }
}
