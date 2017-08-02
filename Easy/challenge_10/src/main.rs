/*
The exercise today asks you to validate a telephone number, as if written on an input form. 
Telephone numbers can be written as ten digits, or with dashes, spaces, or dots between the 
three segments, or with the area code parenthesized; both the area code and any white space 
between segments are optional.

valid telephone numbers: 
1234567890, 
123-456-7890, 
123.456.7890, 
(123)456-7890, 
(123) 456-7890 (note the white space following the area code), and 
456-7890.

not valid telephone numbers: 
123-45-6789, 
123:4567890, and 
123/456-7890. 



My regex string:
^(?:\(\d{3}\)|\d{3})?[.|\-| ]?\d{3}[.|\-]?\d{4}$

Matches:
^                       : the start of the string
(?:a|b)?                : Either a or b or neither (see the next two lines for a and b)
\(\d{3}\)               : 3 digits surrounded by parentheses
\d{3}                   : 3 digits
[.|\-| ]?               : . or - or space or nothing
\d{3}                   : 3 digits
[.|\-]?                 : . or - or nothing
\d{4}                   : 4 digits
$                       : the end of the string

If you don't wrap the regex in ^$ , then partial matches will match (such as qwerty4567890)
*/

extern crate regex;

use regex::Regex;
use std::io;

fn test_phone(phone_num: &str) -> bool {
    let re = Regex::new(r"^(?:\(\d{3}\)|\d{3})?[.|\-| ]?\d{3}[.|-]?\d{4}$").unwrap();
    re.is_match(phone_num)
}

fn main() {
    println!("Enter a phone number: ");
    let mut num = String::new();

    // There's another way to handle this (other than nested `match` statements) with something like `and_then()`
    match io::stdin().read_line(&mut num) {
        Ok(_)  => {
            match test_phone(num.trim()) {
                true => println!("Valid phone number"),
                false => println!("Invalid phone number"),
            }
        },
        Err(e) => println!("{}", e),
    }
}


#[test]
fn test_phone_test() {
    // Valid phone numbers
    assert!(test_phone("1234567890"));
    assert!(test_phone("123-456-7890"));
    assert!(test_phone("123.456.7890"));
    assert!(test_phone("(123)456-7890"));
    assert!(test_phone("(123) 456-7890"));
    assert!(test_phone("456-7890"));

    // Invalid phone numbers
    assert!(!test_phone("123-45-6789"));
    assert!(!test_phone("123:4567890"));
    assert!(!test_phone("123/456-7890"));
}