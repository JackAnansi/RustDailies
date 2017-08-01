/*
    Morse code encrypter / decrypter
*/
use std::collections::HashMap;

fn main() {
    let mut morse = HashMap::new();
    morse.insert(".-", "A");
    morse.insert("-...", "B");
    morse.insert("-.-.", "C");
    morse.insert("-..", "D");
    morse.insert(".", "E");
    morse.insert("..-.", "F");
    morse.insert("--.", "G");
    morse.insert("....", "H");
    morse.insert("..", "I");
    morse.insert(".---", "J");
    morse.insert("-.-", "K");
    morse.insert(".-..", "L");
    morse.insert("--", "M");
    morse.insert("-.", "N");
    morse.insert("---", "O");
    morse.insert(".--.", "P");
    morse.insert("--.-", "Q");
    morse.insert(".-.", "R");
    morse.insert("...", "S");
    morse.insert("-", "T");
    morse.insert("..-", "U");
    morse.insert("...-", "V");
    morse.insert(".--", "W");
    morse.insert("-..-", "X");
    morse.insert("-.--", "Y");
    morse.insert("--..", "Z");

    let example = ".... . .-.. .-.. --- / -.. .- .. .-.. -.-- / .--. .-. --- --. .-. .- -- -- . .-. / --. --- --- -.. / .-.. ..- -.-. -.- / --- -. / - .... . / -.-. .... .- .-.. .-.. . -. --. . ... / - --- -.. .- -.--";
    
    let words: Vec<&str> = example.split(" / ").collect();
    for word in words {
        let letters: Vec<&str> = word.split(" ").collect();
        for letter in letters {
            print!("{}", morse.get(&letter).unwrap());
        }
        print!(" ");
    }
}
