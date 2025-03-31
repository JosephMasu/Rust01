//simple cli in rust

 use std::env;
 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Enter a string to reverse");
        return
    } 

    let input: String = args[1].clone();
    let reversed: String = input.chars().rev().collect::<String>();
    println!("Reversed: {}", reversed);
}
