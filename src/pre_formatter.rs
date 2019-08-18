use std::fmt;

pub struct Menu {
  pub date: String,
  pub dishes: Vec<String>,
}

impl PartialEq for Menu {
  fn eq(&self, other: &Self) -> bool {
    self.date == other.date && self.dishes == other.dishes
  }
}

impl fmt::Debug for Menu {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Menu {{ date: {}, dishes: {:?} }}",
      self.date, self.dishes
    )
  }
}

pub fn pre_format_text(raw_text: String) -> Vec<String> {
  let without_line_breaks: Vec<&str> = raw_text.split_terminator("\r\n").collect();
  let without_empty_strings: Vec<&str> = without_line_breaks
    .into_iter()
    .filter(|x| !x.trim().is_empty())
    .collect();
  let without_white_spaces: Vec<&str> = without_empty_strings
    .into_iter()
    .map(|x| x.trim())
    .collect();
  let without_unneeded_entries: Vec<&str> = without_white_spaces
    .into_iter()
    .filter(|x| x != &"Wochenkarte")
    .filter(|x| x != &"----------------Page (0) Break----------------")
    .collect();

  let as_string_vector: Vec<String> = without_unneeded_entries
    .into_iter()
    .map(|x| x.to_owned())
    .collect();
  return as_string_vector;
}


pub fn split_date_and_dishes(date_and_dishes: Vec<String>) -> Menu {
  let date = date_and_dishes[0].clone();
  let dishes = date_and_dishes.into_iter().skip(1).collect();

  let menu = Menu {
    date: date,
    dishes: dishes,
  };
return menu;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn split_date_and_dishes_test() {
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

    let expected_result = Menu {
      date: String::from("Vom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr"),
      dishes: vec![
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
      ],
    };

    let result = split_date_and_dishes(input);

    assert_eq!(result, expected_result);
  }

  #[test]
  fn pre_format_text_test() {
    let input = String::from(" \r\n \r\n \r\n \r\nWochenkarte \r\n \r\n \r\nVom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr \r\n \r\nMontag \r\n \r\nRahmschnitzel mit Spätzle  €  8.00 \r\n \r\nDienstag \r\n \r\nGyros mit Tzatziki und Pommes €  8,00 \r\n \r\nMittwoch \r\n \r\nHähnchencurry mit Reis  €  8,00 \r\n \r\nDonnerstag \r\n \r\n BBQ   €  9,90  \r\n \r\nFreitag  \r\n \r\nTintenfischtulpen mit Rosmarinkartoffeln  €  8,00   \r\n \r\n            \r\nTäglich \r\n \r\nSalatteller mit Pangasiusfilet  € 7,50 \r\nChilli con Carne mit Brot € 7,00 \r\nTagliatelle mit frischen Pfifferlingen € 8,90 \r\n----------------Page (0) Break----------------\r\n");
    let result = pre_format_text(input);
    let expected_result = vec![
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
    assert_eq!(result, expected_result);
  }
}
