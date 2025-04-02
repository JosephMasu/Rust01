#[allow(dead_code)]
#[derive(Debug)]    
enum Coin{
    Penny,
    Nickel,
    Dime(Rarity),
    Quarter
}
#[allow(dead_code)]
#[derive(Debug)]    
enum Rarity{
    Common,
    Uncommon,
    Rare,
    Legendary
}



fn value_of(coin: Coin) -> i32{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime (rarity) => {
            println!("You have got a {:?} quarter!", rarity);
            25
        }
        Coin::Quarter => 10
    }
}

fn main() {
    let coin = Coin::Penny;
    let coin1 = Coin::Dime(Rarity::Common);
    println!("The value of the coin is {}", value_of(coin));
    println!("The value of the coin is {}", value_of(coin1));
    
}