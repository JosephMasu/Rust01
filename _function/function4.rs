

fn sum_diff(a: i32, b: i32) -> (i32, i32) {
    return(a + b, a - b)
}

fn main() {
    let (sum, diff) = sum_diff(5, 3);
    println!("sum: {}, diff: {}", sum, diff);
}