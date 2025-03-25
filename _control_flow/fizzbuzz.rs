fn main(){
    let count = 1;
    for count in count..=100{
        if count % 3 == 0 && count % 5 == 0
            {println!("FizzBuzz");}
        else if count % 3 == 0
            {println!("Fizz");}
        else if count % 5 == 0
            {println!("Buzz");}
        else
            {println!("{}", count);}
    }
}