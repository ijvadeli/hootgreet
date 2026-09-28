use chrono::{DateTime, Local};

// Using chrono crate to get local time
pub fn today() -> String {
    let date: DateTime<Local> = Local::now();

    // %A = full day name
    // %d = day number
    // %B = full month name
    // %Y = year
    date.format("%A %d %B %Y").to_string()
}
