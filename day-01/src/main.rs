use std::fs::read_to_string;

fn main() {
    //let first = first();
    //println!("first solution: {first}");
    let second = second();
    println!("second solution: {second}")
}

fn first() -> i32 {
    let input = read_to_string("input1").unwrap();
    let mut count = 0;
    for char in input.chars() {
        match char {
            '(' => count += 1,
            ')' => count -= 1,
            _ => println!("found: {char}")
        }
    }
    count
}

fn second() -> usize {
    let input = read_to_string("input1").unwrap();
    let mut count = 0;
    for (i,char) in input.chars().enumerate() {
        match char {
            '(' => count += 1,
            ')' => count -= 1,
            _ => println!("found: {char}")
        }
        if count == -1 {
            return i+1;
        }
    };
    return 0;
}