use colored::Colorize;

// Owl ascii art
const OWL: &str = r#"   , _ ,
  ( o,o )
 /'` - `'\
 |'''''''|
  \\'''//
====w=w====
"#;

// --- Colors ---
// Owl
pub fn print_display(sysdis: &str) {
    let owl: Vec<&str> = OWL.lines().collect();
    let sys: Vec<&str> = sysdis.lines().collect();

    for i in 0..owl.len().max(sys.len()) {
        println!(
            "{:<16}{}",
            owl.get(i).unwrap_or(&"").green(),
            sys.get(i).unwrap_or(&"")
        );
    }
}

// Date
pub fn print_date(date: &str) {
    println!("{}", date);
}

// Quote
pub fn print_quote(quote: &str) {
    println!("{}", quote.green());
}
