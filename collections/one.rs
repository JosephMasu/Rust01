

fn main(){
    let vec1 = vec![1,2,3];
    let vec2: Vec<i32> = Vec::new();
    let mut vec3 = vec![1,2,3];
    
    vec3.push(4);
    vec3.push(5);
    vec3.push(6);

    vec3.pop();
    
    println!("Vector3 {:?}", vec3);
    println!("Vector {:?}", vec1);
    println!("Vector {:?}", vec2);
     
    //reading element

    let vec4 = vec!['a', 'b', 'c', 'd', 'e'];
    let fourth:&char = &vec4[3];
    println!("the fourht element is {}", fourth);

    //using get method

    let fifth:Option<&char> = vec4.get(5);
    match fifth{
        Some(&fifth) =>println!("the fifth element is {}", fifth),
        None =>println!("There is no fifth element"),
    }

    let vec5 = vec!['a', 'b', 'c', 'd', 'e'];
    for i in &vec5{
        println!("{}", i);
    }

    let mut vec6 = vec![10, 20, 30, 40, 50];
    for i in &mut vec6{
        *i += 10;
    }
    println!("{:?}", vec6);

    #[derive(Debug)]

    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Int(9),
    ];

    println!("{:?}", row);

}