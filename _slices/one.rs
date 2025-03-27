
fn main(){
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..4];
    println!("{:?}", slice);


    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &vec[3..4];
    println!("{:?}", slice);

    let word: String = String::from("Hello world");
    let slice1: &str = &word[0..5];
    let slice2: &str = &word[6..11];
    println!("{:?}", slice1);


    println!("{:?}", slice2);
}
    
