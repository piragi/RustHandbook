enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
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
