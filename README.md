# Rust Tresor Ticker

This is a Rust reimplementation of the [Tresor Ticker](https://github.com/friedhelmensch/tresorticker).

The service is hoted on a "serverless" lambda at [zeit.co](https://zeit.co/blog/introducing-now-rust).

See it in action at [https://tresor-ticker-rust.friedhelmensch.now.sh](https://tresor-ticker-rust.friedhelmensch.now.sh)

## Current issues

The pdf to text crate in use [pdf-extract](https://github.com/jrmuizel/pdf-extract) does not seem to be able to parse PDF file correctly. The output is always empty. Other, simpler files do work though...

The workaround is to use a web service that downloads the PDF and converts it to text instead of using a crate.
