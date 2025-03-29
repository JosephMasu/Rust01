
# [derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}
# [derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary
}

fn main(){
    let coin = Coin::Quarter(Rarity::Common);

    if let Coin::Quarter(rarity)=coin{
        println!("This quarter is {:?}!", rarity)
    } else{
        println!("Not a quarter")
    }
}