//expression is anything that returns a value
//Statement is anything that does not return a value

fn main() {
    let x: i32={
        let price =5;
        let qty=10;
        price * qty
    };
    println!("Total cost: {}", x);
    let sum = add(9, 6);
    println!("The value of the sum is: {}", sum);
    println!("The value  from thr function  is: {}", add(9,6));


    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("The value of the BMI is: {:.3}", bmi);


}
//Functions return a value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//Final example
//BIM = weight(kg)/ height(m)squared 2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}


