/*
    Create random passwords for the user
*/

extern crate rand;

use std::io;
use rand::Rng;

fn do_thing(num: u32, size: u32) -> Vec<Vec<char>> {
    let mut all: Vec<Vec<char>> = Vec::new();
    for _ in 0 .. num {
        let pass: Vec<char> = rand::thread_rng().gen_ascii_chars().take(size as usize).collect();
        all.push(pass);
    }
    all
}

fn main() {
    let mut num_passwords = String::new();
    let mut size_passwords = String::new();

    println!("How many passwords would you like to generate?");
    io::stdin().read_line(&mut num_passwords).unwrap();
    println!("How many characters should each password have?");
    io::stdin().read_line(&mut size_passwords).unwrap();

    let passwords = do_thing(num_passwords.trim().parse::<u32>().unwrap(), size_passwords.trim().parse::<u32>().unwrap());
    for pass in passwords {
        let p: String = pass.iter().clone().collect();
        println!("{}", p);
    }
}
