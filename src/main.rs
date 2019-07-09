use http::{self, Request, Response, StatusCode};
use reqwest::header;

fn handler(request: Request<()>) -> http::Result<Response<String>> {

  let test = String::from("Hallo World (Frank)");

  let response = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(test)
    .expect("failed to render response");

  Ok(response)
}
