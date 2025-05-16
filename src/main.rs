use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Datelike, Timelike};
use std::io::{self, Write};

fn main() {
    print!("Enter date in format like this: 'Fri 16 May 15:00': ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Split into parts
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 4 {
        println!("Invalid format. Use: 'Fri 16 May 15:00'");
        return;
    }

    let day = parts[1].parse::<u32>().unwrap();
    let month = match parts[2] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => {
            println!("Invalid month abbreviation.");
            return;
        }
    };

    let time_parts: Vec<u32> = parts[3]
        .split(':')
        .map(|x| x.parse().unwrap())
        .collect();

    let hour = time_parts[0];
    let minute = time_parts[1];

    // Assume current year
    let year = 2025;
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
    let datetime = NaiveDateTime::new(date, time);

    let timestamp = format!(
        "{:04}{:02}{:02}{:02}{:02}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute()
    );

    println!("Formatted timestamp: {}", timestamp);
}
