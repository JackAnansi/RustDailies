/*
    Sorts a user's digits

    Example:
    6
    [6]
    2
    [2, 6]
    8
    [2, 6, 8]
    4
    [2, 4, 6, 8]
    4
    [2, 4, 4, 6, 8]
*/

use std::io;

fn main() {
    let mut digits = vec![];

    loop {
        let mut d = String::new();
        io::stdin().read_line(&mut d);
        digits.push(d.trim().parse::<i32>().unwrap());

        digits.sort();
        println!("{:?}", digits);
    }
}
