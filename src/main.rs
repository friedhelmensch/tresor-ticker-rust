use http::{self, Request, Response, StatusCode};
use reqwest::header;

fn main() {
 
 let request = Request::builder()
    .method("GET")
    .uri("https://www.rust-lang.org/")
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();

  let result = handler(request);
  let response = result.unwrap();

  println!("{}", response.body());
  println!("{}", response.body());
}

fn handler(request: Request<()>) -> http::Result<Response<String>> {
  let test = String::from("Soon you will see the tresor menu here");

  let response = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(test)
    .expect("failed to render response");

  Ok(response)
}
