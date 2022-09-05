fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if number < 5 { number } else { 0 };
    println!("number is {number}");

    let mut counter = 0;

    let result = 'counting_up: loop {
        counter += 1;
        loop {
            counter += 2;
            if counter >= 10 {
                break 'counting_up counter * 2;
            }
        }
    };

    println!("result is {result}");
}
