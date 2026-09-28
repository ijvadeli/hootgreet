mod date;
mod quotes;
mod ui;

fn main() {
    println!("{}", ui::owl());

    let date = date::today();
    ui::print_date(&date);

    let quote = quotes::random();
    ui::print_quote(&quote);
}
