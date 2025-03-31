use std::env;


fn main() {

    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let args1: &String = &args[1];
    let args2: &String = &args[2];
    
    println!("agrs1: {}", args1);
    println!("agrs2: {}", args2);
    
}
