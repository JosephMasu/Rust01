
struct Person {
    name: String,
    age: u8,
    address: String,
    
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let student = Person{
        name: String::from("John"),
        age: 25,
        address: String::from("Kampala"),
    };
    println!("Name: {}", student.name);
    println!("Age: {}", student.age);
    println!("Address: {}", student.address);

    let light = TrafficLight::Red;
    
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}