//Exercise 2: Fibonacci Sequence
//Write a Rust function that generates the Fibonacci sequence up to the nth term using loops. Take the value of n as input and print the sequence.

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).expect("Unable to read data");
    let num: u32 = (&input.trim()).parse().unwrap();

    let mut first: u32 = 0; 
    let mut second: u32 = 1;

    println!("{}", first);
    println!("{}", second);

    for _i in 1..num-1 {
        let sum = first + second;
        first = second;
        second = sum;
        println!("{}", sum);
    }

}