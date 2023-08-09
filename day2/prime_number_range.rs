//Exercise 3: Prime Number Range
//Write a Rust program that takes two numbers as input, start and end, and prints all prime numbers in the range [start, end].

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut start: String = String::new();
    let mut end: String = String::new();
    stdin.read_line(&mut start).expect("Unable to read line");
    stdin.read_line(&mut end).expect("Unable to read line");
    let s: u32 = (&start.trim()).parse().unwrap();
    let e: u32 = (&end.trim()).parse().unwrap();
    for i in s..=e {
        let mut prime: bool = true;
        let sqrt: u32 = (i as f64).sqrt() as u32;
        for j in 2..=sqrt {
            
            if i%j == 0 {
                prime = false;
                break;
            }
        }
        
        if prime == true {
            println!("{}", i);
        }
    }

}