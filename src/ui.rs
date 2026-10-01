use colored::Colorize;

// Owl ascii art
const OWL: &str = r#"
   , _ ,
  ( o,o )
 /'` - `'\
 |'''''''|
  \\'''//
====w=w====
"#;

// --- Colors ---
// Owl
pub fn owl() -> String {
    OWL.to_string()
}

// Date
pub fn print_date(date: &str) {
    println!("{}", date);
}

// Quote
pub fn print_quote(quote: &str) {
    println!("{}", quote.green());
}

// System info
// pub fn print_system(sysdis: &str) {
//     println!("{}", sysdis.green());
// }
