use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn main() {
    let fst = first();
    println!("Fst: {fst}");
    let snd = second();
    println!("Snd: {snd}")
}

fn second()->i32{
    let input = read_to_string("input.txt").unwrap();
    let mut nice_count = 0;

    for line in input.lines(){
        if is_str_nice_2(line) {
            nice_count += 1;
        }
    }
    return nice_count;
}

fn is_str_nice_2(s:&str)->bool{
    if !s.is_ascii(){
        println!("VORSICHT");
    }
    let s = s.as_bytes();
    let mut pair_occured = false;
    let mut triple_occured = false;
    let mut pairs_occured:HashMap<&[u8], usize> = HashMap::new();
    for (i, c) in s.iter().enumerate() {
        if i >= 3 {
            if let Some(j) = pairs_occured.get(&s[i-1..i+1]) {
                if *j <= i-2 {
                    pair_occured = true;
                }
            }
        }
        if i >= 1 {
            if !pairs_occured.contains_key(&s[i-1..i+1]){
                pairs_occured.insert(&s[i-1..i+1], i);
            }
        }
        if i >= 2 && c == &s[i-2] {
            triple_occured = true;
        }
    }
    return triple_occured && pair_occured;
}

fn first()->i32{
    let input = read_to_string("input.txt").unwrap();
    let mut nice_count = 0;

    for line in input.lines(){
        if is_str_nice_1(line) {
            nice_count += 1;
        }
    }
    return nice_count;
}

fn is_str_nice_1(s:&str)->bool{
    let mut vowel_count = 0;
    let mut last_letter = '.';
    let mut doubeled_letter = false;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => ()
        }
        if last_letter == c {
            doubeled_letter = true
        }
        match last_letter {
            'a' => if c == 'b' {return false;},
            'c' => if c == 'd' {return false;},
            'p' => if c == 'q' {return false;},
            'x' => if c == 'y' {return false;},
            _ => ()
        }
        last_letter = c;
    }
    return vowel_count >= 3 && doubeled_letter;
}