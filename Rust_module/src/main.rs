
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main(){
    let plant: Asparagus = Asparagus{
        name: String::from(
            "Asparagus"
        ),
        color: String::from(
            "Green")
        ,
        stalks: 5,
    };

    println!("I'm growing {:?}!", plant);
}