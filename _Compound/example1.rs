// compound data types
// arrays, tuples, slices, and strings (slice string)

//Array types and strings 

fn main() {
    let numbers:[i32; 5] = [1, 2, 3, 4, 5];
    let fruits:[&str; 3] = ["Apples", "Bananas", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);
    println!("numbers {:?}", numbers); 
    println!("numbers[0]: {}", numbers[0]);
    println!("numbers {:#?}", numbers); 

    //tuples

    let person: (String ,i32, bool) = ("Muhindo".to_string(), 30, false);
    println!("person tuple: {:?}", person);


    //string and string Slice
    
        // `String`: Owned, mutable
        let mut owned_string = String::from("Hello");
        owned_string.push_str(", world!"); // Modify the String
        println!("Owned String: {}", owned_string);
    
        // `&str`: Borrowed, immutable
        let string_slice: &str = &owned_string[0..5]; // Borrow a slice (first 5 characters)
        println!("String Slice: {}", string_slice);
    
        // Passing to a function
        print_message(&owned_string); // Pass as &str (borrowed)
    
    
    fn print_message(message: &str) {
        println!("Message: {}", message); // Works with &str
    }
    
}

