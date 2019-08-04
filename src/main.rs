use http::{self, Request, Response, StatusCode};
use reqwest::header;

use lopdf;
use pdf_extract;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::path::PathBuf;

fn main() {

  let file = "Wochenkarte.pdf".to_owned();
  let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());

  println!("{}", file);
  println!("{}", output_kind);

  let path = path::Path::new(&file);
  let filename = path.file_name().expect("expected a filename");

  let mut output_file = PathBuf::new();
  output_file.push(filename);
  output_file.set_extension(&output_kind);

  let mut output_file = BufWriter::new(File::create(output_file).expect("could not create output"));
  let doc = lopdf::Document::load(path).unwrap();

  pdf_extract::print_metadata(&doc);

  //no text is extracted
  let extracted_text = doc.extract_text(&[1]);
  println!("{:?}", extracted_text);

  let mut output: Box<pdf_extract::OutputDev> = match output_kind.as_ref() {
    "txt" => Box::new(pdf_extract::PlainTextOutput::new(
      &mut output_file as (&mut std::io::Write),
    )),
    "html" => Box::new(pdf_extract::HTMLOutput::new(&mut output_file)),
    "svg" => Box::new(pdf_extract::SVGOutput::new(&mut output_file)),
    _ => panic!(),
  };

  pdf_extract::output_doc(&doc, output.as_mut());

  /*
  let request = Request::builder()
    .method("GET")
    .uri("https://www.rust-lang.org/")
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();

  let result = handler(request);
  let response = result.unwrap();

  println!("{}", response.body());
  */
}

fn handler(_request: Request<()>) -> http::Result<Response<String>> {
  let test = String::from("Soon you will see the tresor menu here");

  let response = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(test)
    .expect("failed to render response");

  Ok(response)
}
