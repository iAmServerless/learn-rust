fn main() {
    let fact = factorial(5);
    println!("{}", fact);
}

fn factorial(n: i32) -> i32 {
    if n == 1 {return 1;}

    n*factorial(n-1)

}