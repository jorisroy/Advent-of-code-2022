use std::fs;
mod cargo;

fn read_input_file() -> String {
    let f: String = fs::read_to_string("src/input.txt").unwrap();
    return f;
}

fn main() {
    let input_file = read_input_file();

    let input_lines = input_file.lines().collect::<Vec<&str>>();

    let instructions: Vec<(usize, usize,usize)> = input_lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            (
                parts[1].parse::<usize>().unwrap(),
                parts[3].parse::<usize>().unwrap() - 1,
                parts[5].parse::<usize>().unwrap() - 1,
            )
        }).collect();
    
    let part_one = cargo::move_crates(&instructions, true)
        .iter()
        .map(|stack| {
        let char = stack.last().unwrap().clone();
        return char;
    }).collect::<String>();

    let part_two = cargo::move_crates(&instructions, false)
        .iter()
        .map(|stack| {
        let char = stack.last().unwrap().clone();
        return char;
    }).collect::<String>();
    
    println!("Answer part 1: {}", part_one);
    println!("Answer part 2: {}", part_two);

}
