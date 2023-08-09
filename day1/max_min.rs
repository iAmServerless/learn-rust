
fn main() {
    let array: [i32; 5] = [5,4,1,7,9];
    let (min, max) = find_max_min(array);
    println!("{} {}", min, max);
}

fn find_max_min(array: [i32; 5]) -> (i32, i32) {
    let mut min: i32 = i32::MAX;
    let mut max: i32 = i32::MIN;
    for value in array {
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }
    (min, max)
}