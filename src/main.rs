#[allow(dead_code)]
mod day_1;
mod day_2;
fn main() {
    // let input = day_1::read_input_file("inputs/day_1/puzzle_1.txt".to_string()).unwrap();
    // let result_1 = day_1::part_1(&input);
    // assert_eq!(result_1, 1029);
    // let result_2 = day_1::part_2(&input);
    // assert_eq!(result_2, 5892);
    let day2_result: i64 = day_2::day_2_pt_1();
    assert_eq!(day2_result, 29818212493);
    let day2_result: i64 = day_2::day_2_pt_2();
    assert_eq!(day2_result, 37432260594);
}
