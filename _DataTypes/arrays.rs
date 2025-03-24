fn main(){
    let arr = [1,2,3,4,5];

    let first_arr = [0];
    let second_arr = [1];

    println!("first_arr: {:?}", first_arr);
    println!("second_arr: {:?}", second_arr);

    for element in arr.iter(){
        println!("element: {}", element);
        
    }

    
}