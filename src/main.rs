use http::{self, Request, Response, StatusCode};
use reqwest;

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

fn handler(_request: Request<()>) -> http::Result<Response<String>> {
  let menu_response = reqwest::get("https://tresormenuservice.friedhelmensch.now.sh/")
    .unwrap()
    .text()
    .unwrap();

  let response = Response::builder()
    .status(StatusCode::OK)
    .body(menu_response)
    .expect("failed to render response");

  Ok(response)
}

