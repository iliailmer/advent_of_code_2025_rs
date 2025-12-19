use std::ops::Rem;
pub fn part_1(input: &str) -> i32 {
    let mut result = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction = line
            .chars()
            .next()
            .map(|c| match c {
                'L' => -1,
                'R' => 1,
                _ => 0,
            })
            .unwrap();
        let step: i32 = line.get(1..).unwrap().parse().unwrap();
        dial += direction * step;
        dial %= 100;

        if dial == 0 {
            result += 1;
        }
    }
    result
}

pub fn part_2(input: &str) -> i32 {
    let mut result = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction: i32 = line
            .chars()
            .next()
            .map(|c| match c {
                'L' => -1,
                'R' => 1,
                _ => 0,
            })
            .unwrap();
        let step: i32 = line.get(1..).unwrap().parse().unwrap();
        let quot: i32 = direction.div_euclid(100);
        let rem = direction.rem(100);

        if direction < 0 {
            result += quot;
            if dial > 0 && dial - rem < 0 {
                result += 1;
            }
            dial += direction * step;
        }
        if direction > 0 {
            if dial < 100 && dial + step > 100 {
                result += quot;
            }
            if dial < 100 && dial + rem > 100 {
                result += 1;
            }
            dial += direction * step;
        }

        dial %= 100;

        if dial == 0 {
            result += 1;
        }
    }
    result
}
