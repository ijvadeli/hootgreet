use chrono::{DateTime, Local};
use colored::Colorize;
use rand::prelude::*;

fn random_quote(quotes: &[&str]) -> String {
    quotes.choose(&mut rand::rng()).unwrap().to_string()
}

fn main() {
    // Owl ascii art
    let owl = r#"
   , _ ,
  ( o,o )
 /'` - `'\
 |'''''''|
  \\'''//
====w=w====
        "#;
    // Define current date using chrono crate
    let date_display: DateTime<Local> = Local::now();
    let formatted_date = date_display.format("%A %d %B %Y").to_string();

    // quotes
    let quotes =
        vec!["All we have to decide is what to do with the time that is given us.\n- Gandalf"];

    // Use random quote function
    let quote = random_quote(&quotes);

    // Print everything
    println!("{}", owl.yellow());
    println!("{}", formatted_date.blue());
    println!("{}", quote.yellow());
}
