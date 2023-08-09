fn main() {
    let is_prime: bool = check_is_prime(13);
    println!("{}", is_prime);
}

fn check_is_prime(num: i32) -> bool {
    let mut is_prime: bool = true;
    for i in 2..=num/2 {
        //println!("{:?}", i);
        if num%i == 0 {
            is_prime = false
        }
    }
    is_prime
}