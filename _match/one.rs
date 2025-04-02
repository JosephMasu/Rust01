fn main(){
    let number = -5;
    match number {
        n if n == 0 => println!("Number is zero"),
        n if n > 0 => println!("Number is positive"),
        n if n < 0 => println!("Number is negative"),
        _ => println!("Invalid number"),
    }  
}

fn main(){
    let number = -5;
    if number == 0 {
        println!("Number is zero");
    } else if number > 0 {
        println!("Number is positive");
    } else {
        println!("Number is negative");
    }
}