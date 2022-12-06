use std::fs;

fn read_starting_order_file() -> String {
    let f: String = fs::read_to_string("src/starting_order.txt").unwrap();
    return f;
}

fn get_current_position() ->[Vec<char>; 9] {
    let cargo_string = read_starting_order_file();
    let cargo_lines = cargo_string.lines().collect::<Vec<&str>>();

    let mut original_stack: [Vec<char>; 9] = Default::default();
    for line in cargo_string.lines() {
        let layer = line;
        if !layer.contains("[") {
            break;
        }

        for line in cargo_lines.iter().rev() {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    original_stack[i].push(c);
                }
            }
        }
    }

    return original_stack;

}

pub fn move_crates(instructions: &Vec<(usize, usize, usize)>, reversed: bool) -> [Vec<char>; 9] {
    let mut stacks = get_current_position();
    for (size, from, to) in instructions {
        let moving_it= stacks[*from].drain(stacks[*from].len() - size..);
        let moving: Vec<char> = if reversed {moving_it.rev().collect()} else {moving_it.collect()};
        stacks[*to].extend(moving);
    }
    return stacks;
}