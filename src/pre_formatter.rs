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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
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
