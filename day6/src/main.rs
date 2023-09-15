use std::{array, collections::HashSet, hash::Hash};

const MARKER_LENGTH: usize = 14;
fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    content.lines().for_each(|line| {
        let mut marker: [char; MARKER_LENGTH] = [' '; MARKER_LENGTH];
        for (i, f) in line.chars().into_iter().enumerate() {
            let index = i % MARKER_LENGTH;
            marker[index] = f;
            let set: HashSet<char> = HashSet::from_iter(marker.iter().cloned());
            if set.len() == MARKER_LENGTH && i >= MARKER_LENGTH {
                println!("{}", i + 1);
                break;
            }
        }
    })
}
