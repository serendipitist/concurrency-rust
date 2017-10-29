extern crate curl;

use curl::easy::Easy;
use curl::easy::List;
use std::io::stdout;
use std::io::Write;


fn main() {
  let url = "https://api.github.com/search/repositories?q=rust";

  let mut easy = Easy::new();

  let mut header = List::new();

  header.append("User-Agent: curl").unwrap();

  easy.http_headers(header).unwrap();

  easy.url(url).unwrap();
  easy.write_function(|data| {
    Ok(stdout().write(data).unwrap())
  }).unwrap();
  easy.perform().unwrap();
}