use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

mod tests;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            let characters = input
                .join("")
                .graphemes(true)
                .map(|g| char::from_str(g.to_lowercase().as_str()).unwrap())
                .filter(|g| g.is_alphabetic())
                .collect::<Vec<char>>();
            HashSet::<&char>::from_iter(characters.iter())
                .par_iter()
                .map(|&&c| (c, characters.iter().filter(|&&ch| ch == c).count()))
                .collect()
        })
}
