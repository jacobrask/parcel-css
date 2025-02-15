fn main() {
  parse();
}

#[cfg(feature = "serde")]
fn parse() {
  use parcel_css::stylesheet::{ParserOptions, StyleSheet};
  use std::{env, fs};

  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1]).unwrap();
  let stylesheet = StyleSheet::parse(&args[1], &contents, ParserOptions::default()).unwrap();
  let json = serde_json::to_string(&stylesheet).unwrap();
  println!("{}", json);
}

#[cfg(not(feature = "serde"))]
fn parse() {
  panic!("serde feature is not enabled")
}
