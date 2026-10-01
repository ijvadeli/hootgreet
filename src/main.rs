mod date;
mod quotes;
mod system;
mod ui;

fn main() {
    // Print owl + system info
    let date = date::today();
    let sysdis = system::systeminfo();
    ui::print_display(&date, &sysdis);

    // Print quote
    let quote = quotes::random();
    ui::print_quote(&quote);
}
