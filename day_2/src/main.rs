use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
mod calculators;


fn read_input_file() -> String {
    let mut data = String::new();
    let mut f = File::open("src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

fn calculate_points_contest(lines: std::str::Lines) {
    let opponent_choices: HashMap<&str, &str> = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors")
    ]);

    let player_choices: HashMap<&str, &str> = HashMap::from([
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors")
    ]);


    let mut points_part_one: i32 = 0;
    let mut points_part_two: i32 = 0;

    for line in lines {
        let splitted_line: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

        let opponent_played: &str = &splitted_line[0];
        let player_played: &str = &splitted_line[1];

        let opponent_choice: &&str = opponent_choices.get(opponent_played).unwrap();
        let player_choice: &&str = player_choices.get(player_played).unwrap();

        points_part_one += calculators::calculate_regular_score(player_choice, opponent_choice);
        points_part_two += calculators::calculate_weird_score(player_choice, opponent_choice);
    }
    println!("Part one: {}", points_part_one);
    println!("Part one: {}", points_part_two);

}

fn main() {

    let file = read_input_file();

    let lines = file.lines();

    calculate_points_contest(lines);
}
