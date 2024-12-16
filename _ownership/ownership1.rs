//Each value in Rust has a variable that's its owner.

fn main() {
    let s1 = String:: from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    //there can only be one owner at a time
    let s2 = s1;
    // println!("The length of '{}' is {}", s1, len); there can only be one owner at a time
    println!("The length of '{}' is {}", s2, len);

}


    //when the owenr goes out of scope, the value will be dropped
    fn print_lost_value(s: &String) {
        println!("The value of s is: {}", s1);
    }
    //local variable with a similar name exists: `s`

fn calculate_length(s: &str) -> usize {
    s.len()
}