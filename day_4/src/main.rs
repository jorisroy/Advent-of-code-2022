use std::fs;
use std::time::Instant;


fn read_input_file() -> String {
    let f: String = fs::read_to_string("src/input.txt").unwrap();
    return f;
}

fn main() {
    let now: Instant = Instant::now();

    let input_string = read_input_file();

    let lines = input_string.lines().collect::<Vec<&str>>();

    let count_overlap: usize = lines
        .iter()
        .map(|line| {
            let jobs: Vec<&str> = line.split(',').collect();
            let job_1_splitted: Vec<i32> = jobs[0].split('-').map(|job| job.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let job_2_splitted: Vec<i32> = jobs[1].split('-').map(|job| job.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            if  
                job_1_splitted[0] >= job_2_splitted[0] && job_1_splitted[1] <= job_2_splitted[1] ||
                (job_2_splitted[0] >= job_1_splitted[0] && job_2_splitted[1] <= job_1_splitted[1])
            {
                return 1
            }
            return 0;
        })
        .sum::<usize>();

    println!("Answer part 1: {}", count_overlap);

    let count_overlap_all: usize = lines
        .iter()
        .map(|line| {
            let jobs: Vec<&str> = line.split(',').collect();
            let job_1_splitted: Vec<i32> = jobs[0].split('-').map(|job| job.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let job_2_splitted: Vec<i32> = jobs[1].split('-').map(|job| job.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            if job_1_splitted[0] <= job_2_splitted[1] && job_1_splitted[1] >= job_2_splitted[0] {
                return 1
            }
            return 0;
        })
        .sum::<usize>();


    println!("Answer part 2: {}", count_overlap_all);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
