use clap::Parser;

#[allow(dead_code)]
mod day_1;
mod day_2;
mod day_3;
mod utils;

/// Advent of Code CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Day number (e.g. 1, 2, 3)
    #[arg(long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => {
            let input =
                utils::read_input_file("inputs/day_1/puzzle_1.txt").unwrap_or(String::from(""));
            let input_trimmed = input.trim();
            let day1_result_1: i32 = day_1::part_1(input_trimmed);
            let day1_result_2: i32 = day_1::part_2(input_trimmed); // TODO: something went wrong with the answer
            println!("Day 1, Part 1: {day1_result_1}");
            println!("Day 1, Part 2: {day1_result_2}");
        }
        2 => {
            let input =
                utils::read_input_file("inputs/day_2/puzzle_1.txt").unwrap_or(String::from(""));
            let day2_result_1: i64 = day_2::day_2_pt_1(&input);
            let day2_result_2: i64 = day_2::day_2_pt_2(&input);
            println!("Day 2, Part 1: {day2_result_1}");
            println!("Day 2, Part 2: {day2_result_2}");
        }
        // Add more cases for other days/parts as needed
        _ => {
            eprintln!("Day {} not implemented.", args.day);
        }
    }
}
