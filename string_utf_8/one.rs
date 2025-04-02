

//UTF-8 encoded string
fn main(){
let hello = String::from("Holla");
let hello = String::from("Hollo");
let hello = String::from("Bonjour");
let hello = String::from("Mbote");

//upadate string
let mut s = String::from("Lunch ");
s.push_str("time");
s.push('!');
println!("{}", s);

//concatenation

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
println!("{}", s3);

//concatenation using format!

let str1 = String::from("one");
let str2 = String::from("two");
let str3 = String::from("three");
let with_format = format!("{}-{}-{}", str1, str2, str3);
println!("{}", with_format);

//indexing into string

let word = String::from("RUSTTTTTT");
let idx = &word[0..9];
println!("{}", idx);


for c in word.chars() {
    println!("{}", c);
}

for b in word.bytes() {
    println!("{}", b);
}






}



