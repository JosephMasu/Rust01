use std::fs::File;
use std::io::{self, ErrorKind};

fn main() {
    // Attempt to open the file
    let greeting_from_file: Result<File, io::Error> = File::open("greeting.txt");

    let greeting_file: File = match greeting_from_file {
        // If the file is found, use it
        Ok(file) => file,
        
        // Handle the error
        Err(error) => match error.kind() {
            // If the file is not found, attempt to create it
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(fc) => fc, // If creation succeeds, return the file
                Err(e) => panic!("Problem creating the file: {:?}", e), // If creation fails, panic
            },
            
            // Handle any other error
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // At this point, greeting_file is guaranteed to be a valid file object.
    println!("File successfully opened or created.");

    //alternative approach with if-else
    // let greeting_file: File = File::open("greeting.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("greeting.txt").expect("Problem creating the file");
    //         File::open("greeting.txt").expect("Problem opening the file")
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
}
