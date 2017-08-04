/*
    Calculates the day of the week, given a d/m/y input

    I believe there's an issue with certain dates -- 1/2/2001 fails, for instance
*/

use std::io;

enum DaysOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

fn get_user_input() -> Result<(u32, u32, u32), String> {
    let mut day = String::new();
    let mut month = String::new();
    let mut year = String::new();

    io::stdin().read_line(&mut day).expect("Couldn't parse input");
    io::stdin().read_line(&mut month).expect("Couldn't parse input");
    io::stdin().read_line(&mut year).expect("Couldn't parse input");

    let d = day.trim().parse::<u32>().expect("Couldn't parse input");
    let m = month.trim().parse::<u32>().expect("Couldn't parse input");
    let y = year.trim().parse::<u32>().expect("Couldn't parse input");
    Ok((d, m, y))
}

// Uses the Gaussian algorithm
fn calculate_dow(day: u32, month: u32, year: u32) -> String {
    let Y;              // Year
    if month == 0 || month == 1 {
        Y = year - 1;
    }
    else {
        Y = year;
    }
    let d = day;        // Day
    let m = ((month + 9) % 12) + 1;     // Month
    let y = Y % 100;    // Last two digits of the year
    let c = Y / 100;    // First two digits of the year [century]
    let w = d as f32 + (2.6 * m as f32 - 0.2) + y as f32 + (y as f32 / 4.) + (c as f32 / 4.) - 2. * c as f32;
    let dow = (w % 7.) as u32;

    match dow {
        x if x == DaysOfWeek::Sunday as u32 => "Sunday".to_owned(),
        x if x == DaysOfWeek::Monday as u32 => "Monday".to_owned(),
        x if x == DaysOfWeek::Tuesday as u32 => "Tuesday".to_owned(),
        x if x == DaysOfWeek::Wednesday as u32 => "Wednesday".to_owned(),
        x if x == DaysOfWeek::Thursday as u32 => "Thursday".to_owned(),
        x if x == DaysOfWeek::Friday as u32 => "Friday".to_owned(),
        x if x == DaysOfWeek::Saturday as u32 => "Saturday".to_owned(),
        _ => unreachable!(),
    }
}

fn print_output(day: u32, month: u32, year: u32, day_of_week: String) {
    print!("{:02}/{:02}/{:04} is on a ", day, month, year);
    println!("{}", day_of_week);
}

fn main() {
    let (day, month, year) = match get_user_input() {
        Err(e) => {
            println!("{}", e);
            (0,0,0)
        },
        Ok((a,b,c)) => (a,b,c)
    };
    //let (day, month, year) = get_user_input().unwrap();
    let day_of_week = calculate_dow(day, month, year);
    print_output(day, month, year, day_of_week);
}
