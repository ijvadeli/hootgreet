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
    // .lines() splits values into individual lines
    // .collect puts them in Vec<&str>
    let owl: Vec<&str> = OWL.lines().collect();
    let date: Vec<&str> = date.lines().collect();
    let sys: Vec<&str> = sysdis.lines().collect();

    // Loop through line numbers, find which amount is larger (OWL or system-info)
    // Makes sure owl doesn't get cut off because sys has less lines
    for i in 0..owl.len().max(sys.len()) {
        println!(
            "{:<16}{}{}",
            // .unwrap_or(&"") gives line if exists else give empty string
            // owl line is padded to 16 characters {:<16}
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
