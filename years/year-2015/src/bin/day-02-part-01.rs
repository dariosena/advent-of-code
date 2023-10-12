// --- Day 1: Not Quite Lisp ---
use std::cmp::min;
use year_2015::InputHelper;

fn main() {
    let input = InputHelper::read_input("day-02", "part-01").unwrap();

    //let measurements = input.split_whitespace();
    let mut total = 0;

    for measurements in input.split_whitespace() {
        let mut box_measurements = measurements.splitn(3, 'x');
        let length: u32 = box_measurements.next().unwrap().parse().unwrap();
        let width: u32 = box_measurements.next().unwrap().parse().unwrap();
        let height: u32 = box_measurements.next().unwrap().parse().unwrap();

        let first_side = length * width;
        let second_side = width * height;
        let third_side = height * length;

        let mut smallest_side = min(first_side, second_side);
        smallest_side = min(smallest_side, third_side);

        total += 2 * first_side + 2 * second_side + 2 * third_side + smallest_side;
    }

    println!("{total}");
}
