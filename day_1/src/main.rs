use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn read_input_file() -> String {
    let mut data = String::new();
    let mut f = File::open("src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

fn collect_calories_per_elf(lines: std::str::Lines) -> Vec<i32> {
    let mut current_elf_calories = 0;
    let mut elf_calories_vector = Vec::new();

    for line in lines {
        if line.chars().count() == 0 {
            elf_calories_vector.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }
    elf_calories_vector.sort();

    return elf_calories_vector;
}

fn combine_calories(from: usize, to: usize, elf_calories_vector: Vec<i32>) -> i32 {
    let mut total_calories = 0;

    for i in from..to {
        total_calories += elf_calories_vector[i];
    }

    return total_calories;
}


fn main() {
    let now = Instant::now();

    let file = read_input_file();

    let lines = file.lines();

    let elf_calories_vector = collect_calories_per_elf(lines);

    let total_elfs = elf_calories_vector.len();

    println!("Answer part 1: {}", elf_calories_vector[elf_calories_vector.len() -1]);
    println!("Answer part 2: {}", combine_calories(total_elfs -3, total_elfs, elf_calories_vector));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}