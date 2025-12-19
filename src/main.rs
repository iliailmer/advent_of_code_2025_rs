use clap::Parser;

#[allow(dead_code)]
mod day_1;
mod day_2;
mod day_3;

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
            let day2_result_1: i64 = day_2::day_2_pt_1();
            let day2_result_2: i64 = day_2::day_2_pt_2();
            println!("Day 2, Part 1: {day2_result_1}");
            println!("Day 2, Part 2: {day2_result_2}");
        }
        2 => {
            let day2_result_1: i64 = day_2::day_2_pt_1();
            let day2_result_2: i64 = day_2::day_2_pt_2();
            println!("Day 2, Part 1: {day2_result_1}");
            println!("Day 2, Part 2: {day2_result_2}");
        }
        // Add more cases for other days/parts as needed
        _ => {
            eprintln!("Day {} not implemented.", args.day);
        }
    }
}
