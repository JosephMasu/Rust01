 struct User{
    name: String,
    age: u8,
    email: String,
    is_active: bool
 }

 fn main() {
    let mut user1:User = User{
        name: String::from("Joseph"),
        age: 25,
        email: String::from("joseph@gmail.com"),
        is_active: true
    };
    user1.name = String::from("Pablo");
    user1.email = String::from("pablo@gmail.com");
    
    println!("User1: {} \nage: {} \nemail: {} \nis_active: {}", user1.name, user1.age, user1.email, user1.is_active);

 }