/*
    Create a program that asks the user's name, age, and Reddit username and prints it out
*/
use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut username = String::new();
    
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("You must enter your name");
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("You must enter a valid age");
    println!("Enter your Reddit username: ");
    io::stdin().read_line(&mut username).expect("You must enter a valid username");

    print!("\nYour name is {}, you are {} years old, and your username is {}", name.trim(), age.trim(), username.trim());
}