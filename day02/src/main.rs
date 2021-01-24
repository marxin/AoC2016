use std::fs;

fn position_to_buton(position: &(i32, i32)) -> i32 {
    7 - 3 * position.1 + position.0
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not open input file");
    let mut position = (1, 1);
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let move_names = "URDL";

    for line in input.lines() {
        // println!("Line: {}", line);

        for c in line.chars() {
            let m = &moves[move_names.find(c).unwrap()];
            let previous = position;

            position.0 += m.0;
            position.1 += m.1;

            if position.0 > 2 || position.0 < 0 || position.1 > 2 || position.1 < 0 {
                position = previous;
            }
        }

        println!("{:?}: {}", position, position_to_buton(&position));
    }

    println!("Hello, world!");
}
