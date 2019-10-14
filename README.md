# Rust Tresor Ticker

This is a Rust reimplementation of the [Tresor Ticker](https://github.com/friedhelmensch/tresorticker).

The service is hosted at [zeit.co](https://zeit.co/blog/introducing-now-rust) using [now-rust](https://github.com/mike-engel/now-rust).

See it in action at [https://tresor-ticker-rust.friedhelmensch.now.sh](https://tresor-ticker-rust.friedhelmensch.now.sh)

## Build Status

[![Build Status](https://travis-ci.org/friedhelmensch/tresor-ticker-rust.svg?branch=master)](https://travis-ci.org/friedhelmensch/tresor-ticker-rust)

## Limitations

The pdf to text crate [pdf-extract](https://github.com/jrmuizel/pdf-extract) that I tried does not seem to be able to parse PDF file correctly. The output is always empty. Other, simpler files do work though...

The workaround is to use a web service that downloads the PDF and converts it to text instead of using a crate.
