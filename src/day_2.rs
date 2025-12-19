pub fn day_2_pt_1(puzzle: &String) -> i64 {
    let ranges: Vec<&str> = puzzle.split(',').collect();
    let mut result: Vec<i64> = Vec::new();
    for range in ranges {
        let pair: Vec<i64> = range.split('-').map(|x| x.parse().unwrap_or(0)).collect();
        let start = pair[0];
        let finish = pair[1];
        for each in start..=finish {
            let each_st = each.to_string();
            let len = each_st.len();
            if len % 2 == 0 {
                let middle = len / 2;
                let first_half = &each_st[..middle];
                let second_half = &each_st[middle..];
                if first_half == second_half {
                    result.push(each);
                }
            }
        }
    }
    result.iter().sum::<i64>()
}

pub fn day_2_pt_2(puzzle: &String) -> i64 {
    // let puzzle = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges: Vec<&str> = puzzle.split(',').collect();
    let mut result: Vec<i64> = Vec::new();
    for range in ranges {
        let pair: Vec<i64> = range
            .split('-')
            .map(|x| x.trim().parse().unwrap_or(0))
            .collect();
        let start = pair[0];
        let finish = pair[1];
        for each in start..=finish {
            let each_st = each.to_string();
            let len = each_st.len();
            for pattern_size in 1..=len / 2 {
                if len % pattern_size == 0 {
                    let pattern = &each_st[..pattern_size];
                    let reps = len / pattern_size;
                    if reps >= 2 && pattern.repeat(reps) == each_st {
                        result.push(each);
                        break;
                    }
                }
            }
        }
    }
    result.iter().sum::<i64>()
}
