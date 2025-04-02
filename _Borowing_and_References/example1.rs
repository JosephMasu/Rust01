fn main() {
    let i = 5;
    call_int(i);
    println!("After calling the function, the value of i: {}", i);

    let s = String::from("Hello, world!");
    callstringt(s);
    // println!("After calling the function, the value of s: {}", s);
}

fn call_int(i: i32) {
    println!("The value of i is: {}", i);
    
}

fn callstringt(s: String) {
    println!("The value of i is: {}", s);
}
