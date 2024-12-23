use std::collections::HashMap;

mod input;

fn main() {
    let input = include_bytes!("./input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &[u8]) -> u32 {
    let (mut left_vec, mut right_vec): (Vec<u32>, Vec<u32>) =
        input::parse_input(input).expect("Failed to parse input");

    left_vec.sort();
    right_vec.sort();

    let mut left_iter = left_vec.iter();
    let mut right_iter = right_vec.iter();

    let mut result: u32 = 0;
    loop {
        let l = left_iter.next();
        let r = right_iter.next();

        match (l, r) {
            (Some(l_val), Some(r_val)) => {
                result = result + u32::abs_diff(*l_val, *r_val);
            }
            (None, None) => break,
            _ => continue,
        }
    }

    result
}

fn part_two(input: &[u8]) -> u32 {
    let (left_vec, right_vec): (Vec<u32>, Vec<u32>) =
        input::parse_input(input).expect("Failed to parse input");

    let mut result: u32 = 0;
    let score_map = make_score_map(right_vec);

    for l in left_vec.iter() {
        let sim_score = score_map.get(l).map(|v| *v).unwrap_or(0);
        result = result + (l * sim_score);
    }

    result
}

fn make_score_map(right_vec: Vec<u32>) -> HashMap<u32, u32> {
    let mut score_map = HashMap::new();

    for v in right_vec.iter() {
        let existing = score_map.get(v).map(|v| *v).unwrap_or(0);
        score_map.insert(*v, existing + 1);
    }

    score_map
}
