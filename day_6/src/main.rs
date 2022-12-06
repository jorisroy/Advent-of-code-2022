use std::fs;
use std::time::Instant;

fn read_input_file() -> String {
    let f: String = fs::read_to_string("src/input.txt").unwrap();
    return f;
}

fn main() {
    let now: Instant = Instant::now();

    let radio_signal = read_input_file();
    let radio_signal_bytes = radio_signal.as_bytes();

    
    let start_position_part_one = decrypt_radio_signal(radio_signal_bytes, 4);
    let start_position_part_two = decrypt_radio_signal(radio_signal_bytes, 14);


    println!("Answer part 1: {} ", start_position_part_one);
    println!("Answer part 2: {} ", start_position_part_two);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn decrypt_radio_signal(radio_signal_bytes: &[u8], size_of_signal: usize) -> usize {
    radio_signal_bytes
        .windows(size_of_signal)
        .position(|slice| {
            let mut range = 1..slice.len();
            let mut range_long = slice.len() - 4..slice.len() % 4;

            range.all(|i| !slice[i..]
                .contains(&slice[i - 1])) && 
            range_long.all(|i| !slice[i..]
                .contains(&slice[i - 1]))
        })
        .unwrap()
        + size_of_signal
}

