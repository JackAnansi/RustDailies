/*
    Create a program that is a useful calculator
    Story Point -> Time calculator
    Thanks to: https://mgattozzi.com/avoiding-logic-errors

    Usage:
    timecalc 2h to pts
      > 1pts
    timecalc 13pts to d
      > 3.25d
*/

use std::env;
use std::fmt;
use std::ops::Add;
use Effort::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Effort {
    StoryPoints(f32),
    Hours(f32),
    Days(f32),
}

impl Effort {
    /// Creates a `StoryPoint` from this `Effort`
    pub fn to_storypoints(self) -> Effort {
        match self {
            s @ StoryPoints(_) => s,
            Hours(h)           => StoryPoints(h / 2.),
            Days(d)            => StoryPoints(d * 8. / 2.),     // 1 day = 8 hours
        }
    }

    /// Creates an `Hours` from this `Effort`
    pub fn to_hours(self) -> Effort {
        match self {
            StoryPoints(s) => Hours(s * 2.),
            h @ Hours(_)   => h,
            Days(d)        => Hours(d * 8.),
        }
    }

    /// Creates a `Days` from this `Effort`
    pub fn to_days(self) -> Effort {
        match self {
            StoryPoints(s) => Days(s * 2. / 8.),
            Hours(h)       => Days(h / 8.),
            d @ Days(_)    => d,
        }
    }
}

impl Add for Effort {
    type Output = Effort;

    fn add(self, rhs: Effort) -> Self::Output {
        match (self, rhs) {
            (Hours(a), b @ _) => {
                match b.to_hours() {
                    Hours(b) => Hours(a + b),
                    _ => unreachable!(),
                }
            },
            (Days(a), b @ _) => {
                match b.to_days() {
                    Days(b) => Days(a + b),
                    _ => unreachable!(),
                }
            },
            (StoryPoints(a), b @ _) => {
                match b.to_storypoints() {
                    StoryPoints(b) => StoryPoints(a + b),
                    _ => unreachable!(),
                }
            },
        }
    }
}

impl fmt::Display for Effort {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StoryPoints(s) => write!(fmtr, "{}pts", s),
            Hours(h)       => write!(fmtr, "{}h", h),
            Days(d)        => write!(fmtr, "{}d", d),
        }
    }
}

/// Validates the user's input
/// Returns a tuple containing the value, from type, and to type, or None
fn validate_args(args: &[String]) -> Option<(String, String, String)> {
    if args.len() != 4 {
        return None;
    }
    
    if !args[1].ends_with("d") && !args[1].ends_with("h") && !args[1].ends_with("pts") {
        return None;
    }

    let mut nums = String::new();
    let mut type_from = String::new();
    for letter in args[1].chars() {
        if letter.is_numeric() || letter == '.' {
            nums.push(letter);
        }
        else {
            type_from.push(letter);
        }
    }

    match args[2].as_ref() {
        "to" => {
            // Do nothing
        },
        _ => {
            return None;
        }
    }

    match args[3].as_ref() {
        "d" | "h" | "pts" => {
            // Do nothing
        },
        _ => {
            return None;
        }
    }

    Some((nums, type_from, args[3].to_owned()))
}

/// Prints usage help
fn print_help() {
    println!("Usage:");
    println!("\ttimecalc x[d|h|pts] to [d|h|pts]");
    println!("Example:");
    println!("\ttimecalc 2h to pts");
    println!("\t > 1pts");
}

fn convert(value: String, from_type: String, to_type: String) {
    //println!("Good arguments!");
    //println!("Converting {} from {} to {}", value, from_type, to_type);

    match from_type.as_ref() {
        "d" => {
            let v = Days(value.parse::<f32>().unwrap());
            match to_type.as_ref() {
                "d" => println!("{}", v.to_days()),
                "h" => println!("{}", v.to_hours()),
                "pts" => println!("{}", v.to_storypoints()),
                &_ => println!("Failed to convert"),
            }
        },
        "h" => {
            let v = Hours(value.parse::<f32>().unwrap());
            match to_type.as_ref() {
                "d" => println!("{}", v.to_days()),
                "h" => println!("{}", v.to_hours()),
                "pts" => println!("{}", v.to_storypoints()),
                &_ => println!("Failed to convert"),
            }
        },
        "pts" => {
            let v = StoryPoints(value.parse::<f32>().unwrap());
            match to_type.as_ref() {
                "d" => println!("{}", v.to_days()),
                "h" => println!("{}", v.to_hours()),
                "pts" => println!("{}", v.to_storypoints()),
                &_ => println!("Failed to convert"),
            }
        },
        &_ => {
            println!("Failed to convert");
        }
    }
}

fn main() {

    //timecalc 13pts to days
    //      > 3.25d

    // We expect there to be exactly 4 arguments to run the program normally:
    // The executable name, the input number, 'to', and the output units
    let args: Vec<_> = env::args().collect();

    match validate_args(&args) {
        Some((n, from, to)) => {
            convert(n, from, to);
        },
        None => {
            print_help();
        }
    }
}

#[cfg(test)]
mod test {
    use super::Effort::{Days, Hours, StoryPoints};

    #[test]
    fn basics() {
        let x = Days(10.0);
        assert_eq!(x.to_storypoints(), StoryPoints(40.0));
        println!("{} is {}", x, x.to_storypoints());

        let y = StoryPoints(13.0);
        assert_eq!(y.to_days(), Days(3.25));
        println!("{} is {}", y, y.to_days());

        let z = Hours(4.0);
        assert_eq!(z.to_hours(), Hours(4.0));
        println!("{} is {}", z, z.to_storypoints());
    }
}
