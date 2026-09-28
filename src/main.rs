mod date;
mod quotes;
mod ui;

fn main() {
    // Print owl
    println!("{}", ui::owl());

    // Print date
    let date = date::today();
    ui::print_date(&date);

    // Print quote
    let quote = quotes::random();
    ui::print_quote(&quote);
}
