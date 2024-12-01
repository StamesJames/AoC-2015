use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let first = first();
    println!("First Solution: {first}");
    let second = second();
    println!("Second Solution: {second}");

}

fn first() -> usize {
    let input = read_to_string("input").unwrap();
    let mut visited_locations_santa = HashSet::new();
    let (mut x_santa, mut y_santa) = (0, 0);
    visited_locations_santa.insert((0, 0));
    for char in input.chars() {
        match char {
            '^' => y_santa += 1,
            '<' => x_santa -= 1,
            '>' => x_santa += 1,
            'v' => y_santa -= 1,
            _ => (),
        }
        visited_locations_santa.insert((x_santa, y_santa));
    }
    return visited_locations_santa.len();
}

fn second() -> usize {
    let input = read_to_string("input").unwrap();
    let mut visited_locations = HashSet::new();
    let (mut x_santa, mut y_santa) = (0, 0);
    let (mut x_bot, mut y_bot) = (0, 0);
    let mut santa_turn = true;
    visited_locations.insert((0, 0));
    for char in input.chars() {
        if santa_turn {
            match char {
                '^' => y_santa += 1,
                '<' => x_santa -= 1,
                '>' => x_santa += 1,
                'v' => y_santa -= 1,
                _ => (),
            }
            visited_locations.insert((x_santa, y_santa));
        } else {
            match char {
                '^' => y_bot += 1,
                '<' => x_bot -= 1,
                '>' => x_bot += 1,
                'v' => y_bot -= 1,
                _ => (),
            }
            visited_locations.insert((x_bot, y_bot));
        }
        santa_turn = !santa_turn;
    }
    return visited_locations.len();
}
