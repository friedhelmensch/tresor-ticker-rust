use http::{self, Request, Response, StatusCode};
use reqwest;

mod pre_formatter;

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

fn handler(_request: Request<()>) -> http::Result<Response<String>> {
  let menu_as_raw_text = reqwest::get("https://tresormenuservice.friedhelmensch.now.sh/")
    .unwrap()
    .text()
    .unwrap();

  println!("{:?}", menu_as_raw_text);

  let pre_formatted_text = pre_formatter::pre_format_text(&menu_as_raw_text);
  println!("{:?}", pre_formatted_text);

  let response = Response::builder()
    .status(StatusCode::OK)
    .body(menu_as_raw_text)
    .expect("failed to render response");

  Ok(response)
}
