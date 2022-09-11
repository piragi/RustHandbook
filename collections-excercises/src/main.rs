extern crate core;
use std::collections::HashMap;
use std::io;

fn main() {
    let vector_of_integers = vec![5, 33, 6, 90, 11, 374, 802, 3, 9090, 420, 6, 6, 11];
    let median = median(&vector_of_integers);
    let mode = mode(&vector_of_integers);
    println!("{:?}", vector_of_integers);
    println!("{}", median);
    println!("{}", mode);

    let pig_latin = pig_latin("apple");
    println!("{}", pig_latin);

    department();


}

fn pig_latin(latin_text: &str) -> String {
    let vowels = vec!['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    let mut pig_latin: String = String::from(latin_text);

    if vowels.contains(&latin_text.chars().nth(0).unwrap()) {
        pig_latin.push_str("-hay");
    } else {
        let suffix = format!("{}{}{}", " -", latin_text.chars().nth(0).unwrap(), "ay");
        pig_latin.remove(0);
        pig_latin.push_str(&*suffix);
    }
    pig_latin
}

fn median(vector: &Vec<i32>) -> f64 {
    match vector.is_empty() {
        false => {
            let mut sorted_vector = vector.clone();
            sorted_vector.sort();
            let remainder = sorted_vector.len()%2;
            let median: f64;

            if remainder == 1 {
                median = sorted_vector[sorted_vector.len() / 2] as f64;
            } else {
                median = (sorted_vector[sorted_vector.len() / 2] as f64 + sorted_vector[sorted_vector.len() / 2 - 1] as f64) / 2.0;
            }
            median
        }
        true => 0.0,
    }
}

fn mode(vector: &Vec<i32>) -> i32 {
    match vector.is_empty() {
        true => 0,
        false => {
            let mut number_counter:HashMap<i32, i32> = HashMap::new();

            for element in vector {
                let counter = number_counter.entry(*element).or_insert(1);
                *counter += 1;
            }

            let mut max_pair = (0, 0);
            for element in number_counter {
                if element.1 > max_pair.1 {
                    max_pair.1 = element.1;
                    max_pair.0 = element.0;
                }
            }
            println!("{}", max_pair.1);
            max_pair.0
        }
    }
}

fn department() {

    let mut department = HashMap::new();


    println!("Add persons to department");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();


        match trimmed_input {
            "quit" => break,
            _ => {
                let extracted_info: Vec<&str> = trimmed_input.split_ascii_whitespace().collect();
                if (extracted_info[0].eq("Add")
                    && extracted_info[2].eq("to"))
                    && extracted_info.len() == 4 {
                    department.entry(extracted_info.get(1).unwrap().to_string())
                        .or_insert(extracted_info.get(3).unwrap().to_string());
                }
            }
        }
    }

    println!("{:?}", department);
    all_people_from_department(department);

}

fn all_people_from_department(department: HashMap<String,String>) {
    println!("What department should be outputted?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let department_name = input.trim();

    for (k,v) in department {
        if v.eq(department_name) {
            println!("{} {}", k,v);
        }
    }

}
