mod operations;

use crate::operations::error::Error;
use crate::operations::*;
use chrono::prelude::*;

pub fn get_dishes_of_the_day(raw_text: String) -> Result<Vec<String>, Error> {
    let preformatted_menu = preformatting::pre_format_text(raw_text);
    let menu = preformatting::split_date_and_dishes(preformatted_menu);
    let dishes_of_the_day = extraction::get_menu_by_day(menu.dishes, Utc::now().weekday());

    match dishes_of_the_day {
        Ok(dishes) => Ok(dishes),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn get_dishes_of_the_day_test() {
        //check_if_error_is_returned_if_menu_is_outdated
    }
}
