//Exercise 1: Leap Year Check
//Write a Rust function that takes a year as input and returns true if it's a leap year and false otherwise. A leap year is divisible by 4, but not by 100, unless it's also divisible by 400.
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).expect("Failed to read input");
    let year: u32 = (&input.trim()).parse().unwrap();

    if year%4 == 0 && year%100 != 0 || year%400 == 0 {
        println!("Its a leap year");
    } else {
        println!("Not a leap year");
    }
    
}