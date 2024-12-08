use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

///
///
pub fn parse_file(filecontent: &str) -> (Vec<u64>, Vec<u64>) {
    let mut results: Vec<(u64, u64)> = Vec::with_capacity(1000);
    for line in filecontent.lines() {
        let words: Vec<u64> = line
            .split("   ")
            .map(|v| v.parse::<u64>().unwrap_or_default())
            .collect();
        results.push((words[0], words[1]));
    }

    results.into_iter().unzip()
}

/// Computes the difference between two sorted vectors having the same length.
/// Namely, if the first is [1, 2, 3] and the second [2, 3, 7] the result
/// would be |1-2| + |2-3| + |3-7|.
pub fn difference(left_list: &mut [u64], right_list: &mut [u64]) -> u64 {
    left_list
        .iter_mut()
        .zip(right_list)
        .map(|(a, b)| match a > b {
            true => *a - *b,
            false => *b - *a,
        })
        .sum()
}

pub fn similarity_score(left_list: &mut [u64], right_list: &mut [u64]) -> u64 {
    // Note: this runs in O(3n) in worst case scenario, it might be reduced
    // by noting that the two vectors are sorted beforehand.
    let mut left_counts: HashMap<u64, u64> = HashMap::new();
    for val in left_list.iter_mut() {
        left_counts
            .entry(*val)
            .and_modify(|val| *val += 1)
            .or_insert(1_u64);
    }
    let mut right_counts: HashMap<u64, u64> = HashMap::new();
    for val in right_list.iter_mut() {
        right_counts
            .entry(*val)
            .and_modify(|val| *val += 1)
            .or_insert(1_u64);
    }
    let mut sim_score = 0_u64;
    for key in left_counts.keys() {
        sim_score += match right_counts.get(key) {
            Some(val) => key * left_counts.get(key).unwrap() * val,
            None => 0_u64,
        }
    }
    sim_score
}

fn main() -> Result<(), std::io::Error> {
    // Read the cmdline and loads the file content.
    let args: Vec<String> = args().collect();
    let filepath = &args[1];
    let mut f = File::open(filepath)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // parse the file lines with ''type'': "NUMBER    NUMBER".
    let (mut left_list, mut right_list) = parse_file(&contents);
    left_list.sort();
    right_list.sort();

    // Computes the requested statistics.
    let difference = difference(&mut left_list, &mut right_list);
    let sim_score = similarity_score(&mut left_list, &mut right_list);
    dbg!(difference);
    dbg!(sim_score);
    Ok(())
}
