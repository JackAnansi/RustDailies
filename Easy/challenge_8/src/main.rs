/*
    Prints out the song '99 Bottles of Beer'
*/

use std::fmt;
use std::fmt::Display;

fn main() {
    //basic_beer();
    better_beer();
}

fn basic_beer() {
    for b in (1..100).rev() {
        match b {
            _ if b > 2 => println!("{} bottles of beer on the wall, {} bottles of beer, you take one down, pass it around, {} bottles of beer on the wall!", b, b, (b-1)),
            _ if b > 1 => println!("{} bottles of beer on the wall, {} bottles of beer, you take one down, pass it around, {} bottle of beer on the wall!", b, b, (b-1)),
            _        => println!("{} bottle of beer on the wall, {} bottle of beer, you take one down, pass it around, no more bottles of beer on the wall!", b, b),
        }
        
    }
}

//////

struct Beer {
    num: i32,
}

impl Display for Beer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.num {
            0 => write!(f, "no more bottles"),
            1 => write!(f, "1 bottle"),
            _ => write!(f, "{} bottles", self.num),
        }
    }
}

fn better_beer() {
    let mut beers = vec![];
    for i in (1..100).rev() {
        beers.push(Beer { num: i });
    }

    print!("{} of beer on the wall, {} of beer, you take one down, pass it around, ", beers[0], beers[0]);
    for ref beer in &beers {
        // print second half
        println!("{} of beer on the wall.", beer);
        // print first half
        print!("{} of beer on the wall, {} of beer, you take one down, pass it around, ", beer, beer);
    }
    print!("{} of beer on the wall.", Beer {num: 0});
}