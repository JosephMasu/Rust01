use std::io::stdin;

fn main() {

println!("choose an operator");

let mut choice: String = String::new();
stdin().read_line(&mut choice).expect("Please enter your choice:");

let choice = choice.trim();

if choice !="+" && choice !="-" && choice !="*" && choice !="/" {
    println!("Invalid operator");
    return;
}
    println!("Enter the first number:");
    let mut first_num : String = String::new();
    stdin().read_line(&mut first_num).unwrap_or_default();

    println!("Enter the second number:");
    let mut secod_num: String = String::new();
    stdin().read_line(&mut secod_num).unwrap_or_default();

    let first_num: f32 = first_num.trim().parse::<f32>().unwrap_or_default();
    let secod_num: f32 = secod_num.trim().parse::<f32>().unwrap_or_default();

    let result: f32 = match choice {
        "+" => first_num + secod_num,
        "-" => first_num - secod_num,
        "*" => first_num * secod_num,
        "/" => first_num / secod_num,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("{} {} {} = {}", first_num, choice, secod_num, result);
}
