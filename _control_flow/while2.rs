fn main() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    for num in arr.iter() {
        println!("Number: {}", num);
    }

    while  index < arr.len() {
        println!("Number: {}", arr[index]);
        index += 1;
    }
}
