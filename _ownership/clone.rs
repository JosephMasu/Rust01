fn main(){
    let s1: String = String::from("Hello");
    let s2: String = s1.clone();
    
    let x: i8 = 5;
    let y: i8 = x;
    
    println!("x = {}, y = {}", x, y);
    println!("s1 = {}, s2 = {}", s1, s2);
}