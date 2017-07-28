/*
    Validate a user's credentials with credentials stored in a file
*/

use std::fs;
use std::io;
use std::io::BufRead;

type Username = String;
type Password = String;

#[derive(Debug, PartialEq, Eq)]
struct Credentials {
    username: Username,
    password: Password,
}

impl Credentials {
    fn read_from_file(&mut self, file_name: &str) -> io::Result<()> {
        let f = fs::File::open(file_name)?; // Returns error result on failure automatically
        let file = io::BufReader::new(&f);
        let mut lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
        self.username = lines.remove(0);
        self.password = lines.remove(0);
        Ok(())
    }
}

// Attempts to authenticate the user-provided credentials
// Returns Ok(true) on a match, or Ok(false) on incorrect credentials
// Returns Err(e) on critical failure
fn authenticate(creds: &Credentials) -> io::Result<bool> {
    let mut user = String::new();
    let mut pass = String::new();
    println!("Enter username: ");
    io::stdin().read_line(&mut user);
    println!("Enter password: ");
    io::stdin().read_line(&mut pass);

    if user.trim() == creds.username && pass.trim() == creds.password {
        Ok(true)
    }
    else {
        Ok(false)
    }
}

fn main() {
    // Load password from file
    let path = "./resources/password.txt";
    let mut c: Credentials = Credentials{ username: "".to_owned(), password: "".to_owned()};
    match c.read_from_file(&path) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    }

    // Main loop
    loop {
        println!("\nPlease enter credentials: ");

        match authenticate(&c) {
            Ok(val) => {
                if val {
                    println!("Starting program...");
                    doThing();
                }
                else {
                    println!("Wrong credentials...");
                }
            },
            Err(e) => {
                println!("Critical Error! Exiting: {}", e);
            }
        }
    }
}

fn doThing() {
    for i in 0..10 {
        println!("{}", i);
    }
}