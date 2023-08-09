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