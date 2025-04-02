// struct, tuple

struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let tup:(i32, i32) = (30,50);
    let rect = Rectangle{width: 30,height: 50,
    };
    
    println!("{}", area(tup));
    println!("{}", area1(rect));
    
}
fn area(dimension: (i32, i32)) -> i32{
    dimension.0 * dimension.1
}

fn area1(rectangle: Rectangle) -> u32{
    rectangle.width * rectangle.height
}
