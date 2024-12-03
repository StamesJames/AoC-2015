use md5::{Digest, Md5};

const KEY: &str = "bgvyzdsv";

fn main() {
    let first = first();
    println!("First Solution: {first}");
    let second = second();
    println!("Second Solution: {second}");
}
fn second() -> u128 {
    for i in 1..10_000_000 {
        let num_string = format!("{i}");
        let key = KEY.to_string() + &num_string;
        let mut hasher = Md5::new();
        hasher.update(key);
        let result: [u8; 16] = hasher.finalize().into();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            println!("{:x?}", result);
            return i;
        }
    }
    panic!("no number found")
}

fn first() -> u128 {
    for i in 1..10_000_000 {
        let num_string = format!("{i}");
        let key = KEY.to_string() + &num_string;
        let mut hasher = Md5::new();
        hasher.update(key);
        let result: [u8; 16] = hasher.finalize().into();
        if result[0] == 0 && result[1] == 0 && result[2] <= 0xf {
            println!("{:x?}", result);
            return i;
        }
    }
    panic!("no number found")
}
