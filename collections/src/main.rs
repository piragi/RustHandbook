use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    hashmaps();
    strings();
    vectors();
}

fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Yellow");
    let yellow = scores.get(&team_name);
    scores.insert(team_name, 30);
    scores.entry(String::from("Green")).or_insert(55);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }
}

fn strings() {
    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();
    s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    s.push_str(" with additional data");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //mind the ownership!

    for c in "नमस्ते".chars() {
        println! {"{}", c};
    }
}

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("cell")),
    ];
}
