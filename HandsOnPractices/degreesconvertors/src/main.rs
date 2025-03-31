//coverting celius to fareneight

use std::io::stdin;

fn main() {
    println!("What dou you want to convert?");
    println!("[F] for converting from celius to fareneight");
    println!("[C] for converting from fareneight to celius") ;

    let mut choice: String = String::new();
    stdin().read_line(&mut choice).expect("Please enter your choice:");

    let choice:String = choice.trim().to_uppercase();

    if choice != "F" && choice != "C"{
        println!("Inavalid choice, choose F or C");
        return
    }
    println!("Enter the temperature:");
    let mut temperature: String = String::new();

    stdin().read_line(&mut temperature).unwrap_or_default();

    let temperature: f32 = temperature.trim().parse::<f32>().unwrap_or_default();

    let new_temperature: f32 = match choice.as_str() {
        "F" => (temperature * 9.0 / 5.0) + 32.0,
        "C" => (temperature - 32.0) * 5.0 / 9.0,
        _ => panic!("Invalid choice")
    };

    println!("The new temperature is: {}", new_temperature);


    
}
