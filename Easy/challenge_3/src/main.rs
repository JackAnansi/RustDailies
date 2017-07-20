/*
    Take user input and Caesar Cipher it
*/

use std::io;

fn cipher(text: &str) -> String {
    text.chars().map(|x| {
        match x {
            'A' ... 'M' | 'a' ... 'm' => ((x as u8) + 13) as char,
            'N' ... 'Z' | 'n' ... 'z' => ((x as u8) - 13) as char,
            _ => x,
        }
    }).collect()
}

fn main() {
    println!("Enter a sentence to cipher: ");
    let mut plaintext = String::new();
    match io::stdin().read_line(&mut plaintext) {
        Ok(_) => {
            let ciphertext = cipher(&plaintext);
            println!("{}", ciphertext);
        },
        Err(e) => println!("Failed to read input: {}", e),
    }
}
