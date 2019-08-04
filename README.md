# Rust Tresor Ticker

The idea is to rebuild the current [Tresor Ticker](https://github.com/friedhelmensch/tresorticker) in Rust.

The serice will be hostet on a "serverless" lambda at [zeit.co](https://zeit.co/blog/introducing-now-rust).

## Current issues

The pdf to text library in use [pdf-extract](https://github.com/jrmuizel/pdf-extract) does not seem to be able to parse PDF file correctly. The output is always empty. Other, simpler files do work though...
