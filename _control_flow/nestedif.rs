fn main(){
    let num = 15;

    if num % 2 == 0{
        println!("the number is even");
    }else{
        println!("the number is odd")
    }

    if num > 10 {
        println!("{} the number is greater than 10", num);
    }else if num == 10 {
        println!("{} the number is equal to 10", num );
    }else{
        println!("{} the number is less than 10", num);
    }
}