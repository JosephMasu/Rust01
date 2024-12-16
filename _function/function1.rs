fn main() {
    hello_function();
    tell_height(72);
    human_id("Joe", 25, 182.2);
}
fn hello_function(){
    println!("Hello, Rust!");
}
fn tell_height(height: u32){
    println!("Your height is {} feet", height);
}
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {} is, I am  {} years old and {} feet tall", name, age, height);
}