use std::io;
use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng(); 
    let random_number: u32 = rng.gen_range(1..=100);
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");

    let input_number: u32 = input.trim().parse().unwrap();

    if input_number == random_number {
        println!("Wpw");
    } else if input_number < random_number {
        println!("less");
    } else {
        println!("more");
    }

    println!("{}", random_number);
}