use std::fs::File;
use std::io::Read;


fn main() -> std::io::Result<()> {

    let mut file : File = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(()) 
}