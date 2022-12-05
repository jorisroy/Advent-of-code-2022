use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn read_input_file() -> String {
    let f: String = fs::read_to_string("src/input.txt").unwrap();
    return f;
}

fn get_number_of_points(item: &char) -> usize {
    let map = {
        let mut lower: HashMap<char, usize> = ('a'..='z')
            .into_iter()
            .map(|c| (c, c as usize - 96))
            .collect();

        let upper: HashMap<char, usize> = ('A'..='Z')
            .into_iter()
            .map(|c| (c, c as usize - 38))
            .collect();
        lower.extend(upper);
        lower
    };

    *map.get(item).unwrap()
}

fn main() {
    let now: Instant = Instant::now();

    let input_string = read_input_file();

    let lines = input_string.lines().collect::<Vec<&str>>();

    let count_points: usize = lines
    .iter()
    .map(|line| {
        let (first_slot, second_slot) = line.split_at(line.len() / 2);

        for character in first_slot.chars() {
            if second_slot.contains(character) {
                return get_number_of_points(&character);
            }
        }
        panic!("Nothing found");
    })
    .sum::<usize>();

    println!("Answer part 1: {}", count_points);

    let count_points_2 = lines
    .chunks(3)
    .map(|chunk| {
        for character in chunk[0].chars() {
            if chunk[1].contains(character) && chunk[2].contains(character) {
                return get_number_of_points(&character);
            }
        }
        panic!("Nothing found");
    })
    .sum::<usize>();

    println!("Answer part 2: {}", count_points_2);


    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
