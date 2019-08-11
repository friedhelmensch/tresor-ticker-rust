pub fn pre_format_text(raw_text: &str) -> Vec<&str> {
  let without_linke_breaks: Vec<&str> = raw_text.split_terminator("\r\n").collect();
  let without_empty_strings: Vec<&str> = without_linke_breaks
    .into_iter()
    .filter(|x| !x.trim().is_empty())
    .collect();
  return without_empty_strings;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let input = " \r\n \r\n \r\n \r\nWochenkarte \r\n \r\n \r\nVom 5. August bis 9. August von 11.30 Uhr bis 14.00 Uhr \r\n \r\nMontag \r\n \r\nRahmschnitzel mit Spätzle  €  8.00 \r\n \r\nDienstag \r\n \r\nGyros mit Tzatziki und Pommes €  8,00 \r\n \r\nMittwoch \r\n \r\nHähnchencurry mit Reis  €  8,00 \r\n \r\nDonnerstag \r\n \r\n BBQ   €  9,90  \r\n \r\nFreitag  \r\n \r\nTintenfischtulpen mit Rosmarinkartoffeln  €  8,00   \r\n \r\n            \r\nTäglich \r\n \r\nSalatteller mit Pangasiusfilet  € 7,50 \r\nChilli con Carne mit Brot € 7,00 \r\nTagliatelle mit frischen Pfifferlingen € 8,90 \r\n----------------Page (0) Break----------------\r\n";
    let result = pre_format_text(&input);

    assert!(result.contains(&"Montag "));
  }
}
