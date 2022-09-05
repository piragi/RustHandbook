fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("Value 5: {x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function {x}, Unit Label: {unit_label}");
}

fn five() -> i32 {
    5
}