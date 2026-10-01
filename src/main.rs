mod date;
mod quotes;
mod system;
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

    system::systeminfo();
    // let sysdis = system::systeminfo();
    // ui::print_system(&sysdis);
}
