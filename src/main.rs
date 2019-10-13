use http::{self, Request, Response, StatusCode};
use now_lambda::{error::NowError, lambda, IntoResponse};
use reqwest;
use std::error::Error;

extern crate tresor_ticker_rust;

use chrono::prelude::*;
use json::object;
use json::stringify;

/*
fn main() {
  let request = Request::builder()
    .method("GET")
    .uri("https://this.Is.Just.For.Debugging./Will.Not.Be.Used.When.Deployed.To.Zeit.co")
    .body(())
    .unwrap();

  let result = handler(request);
  let response = result.unwrap();

  println!("{}", response.body());
}
*/

fn main() -> Result<(), Box<dyn Error>> {
  Ok(lambda!(handler))
}

fn handler(_request: Request<()>) -> Result<impl IntoResponse, NowError> {
  let menu_as_raw_text = reqwest::get("https://tresormenuservice.friedhelmensch.now.sh/")
    .unwrap()
    .text()
    .unwrap();

  let pre_formatted_text = tresor_ticker_rust::pre_format_text(menu_as_raw_text);
  let menu = tresor_ticker_rust::split_date_and_dishes(pre_formatted_text);
  let dishes_of_the_day = tresor_ticker_rust::get_menu_by_day(menu.dishes, Utc::now().weekday());

  let html_formatted = dishes_of_the_day
    .iter()
    .fold(String::from(""), |prev, curr| {
      prev + "<li>" + curr + "</li>"
    });

  let html_formatted = format!("{}{}{}", "<ul>", html_formatted, "</ul>");

  let json_return_value = object! {
    "type" => "message",
    "text" => html_formatted
  };

  let return_value = stringify(json_return_value);

  let response = Response::builder()
    .header("Content-Type", "application/json")
    .status(StatusCode::OK)
    .body(return_value)
    .expect("failed to render response");

  Ok(response)
}
