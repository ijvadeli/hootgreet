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
pub fn print_display(date: &str, sysdis: &str) {
    let owl: Vec<&str> = OWL.lines().collect();
    let date: Vec<&str> = date.lines().collect();
    let sys: Vec<&str> = sysdis.lines().collect();

    for i in 0..owl.len().max(sys.len()) {
        println!(
            "{:<16}{}{}",
            owl.get(i).unwrap_or(&"").yellow(),
            date.get(i).unwrap_or(&""),
            sys.get(i).unwrap_or(&"")
        );
    }
}

// Quote
pub fn print_quote(quote: &str) {
    println!("\n{}", quote.yellow());
}
