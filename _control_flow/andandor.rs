fn main() {
    let a = 20;
    let b = 15;
    let c = 9;

    if a > b && b > c{
        println!("&& codition is met")
    } 
    else {println!("condition is not met")}

    if a > b || b == c{
        println!("|| codition is met")
    } 
    else {println!("|| condition is not met")}
}