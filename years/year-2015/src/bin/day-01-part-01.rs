// --- Day 1: Not Quite Lisp ---

use year_2015::InputHelper;

fn main() {
    let input = InputHelper::read_input("day-01", "part-01").unwrap();

    let mut floor = 0;

    for character in input.chars() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }

    println!("{floor}");
}
