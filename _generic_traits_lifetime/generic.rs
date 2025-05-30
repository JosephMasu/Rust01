
fn largest<T : std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
  
    for item in list {
        if item > largest {
            largest = item;
        }
    }
  
    largest
  }

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
