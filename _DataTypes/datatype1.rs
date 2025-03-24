//primirive data types are int, float, bool, char

fn main() {
    // let x = 5; //integer
    // let y = 2.5; //float
    // let z = true; //boolean
    // let a = 'a'; //character

    let tup: (i32, f64, char) = (500, 6.4, 'X');
    let (_x, y, _z) = tup;

    //accessing single tuple element by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x = tup.2;
    println!("The value of x is: {}", y);

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of fx is: {}", x);

    // println!("The value of x is: {}", x);
    // println!("The value of z is: {}", z);
 
    // let d: i8 = -42;
    // let c: u64 = 100;

    // println!("x is: {}", x);
    // println!("y is: {}", y);
    // println!("z is: {}", z);
    // println!("a is: {}", a);
    // println!("Unsigned Integer is: {}", d);
    // println!("Signed Integer is: {}", c);
}