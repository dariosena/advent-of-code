// --- Day 1: Not Quite Lisp ---

use year_2015::InputHelper;

fn main() {
    let input = InputHelper::read_input("day-01", "part-02").unwrap();

    let mut floor = 0;

    for (position, character) in input.chars().enumerate() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            println!("{}", position + 1);
            break;
        }
    }
}
