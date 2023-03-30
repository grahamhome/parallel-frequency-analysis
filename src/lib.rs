use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

mod tests;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            let characters = input
                .join("")
                .chars()
                .map(|c| c.to_lowercase().next().unwrap())
                .filter(|c| c.is_alphabetic())
                .collect::<Vec<char>>();
            HashSet::<&char>::from_iter(characters.iter())
                .par_iter()
                .map(|&&c| (c, characters.iter().filter(|&&ch| ch == c).count()))
                .collect()
        })
}
