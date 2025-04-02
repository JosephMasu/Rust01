fn main(){
    let number =  3;
    if number < 5 {
        println!("condtion was fasle");
    }
    else{
        println!("condtion was true");
    }

    my_function();
}

fn my_function (){
    let condtion = false;
    let number = if condtion {5} else {6};

println!("The value of number is: {}", number);
}