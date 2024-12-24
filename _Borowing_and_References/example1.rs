fn main() {
    let x: i32 = 5;
    let y: &{unknown} = &x;

    println!("The value of x is: {} and the y is {}", x, y);
}