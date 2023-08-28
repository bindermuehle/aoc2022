use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!("Value {}, is not valid", value)),
        }
    }
}
impl Item {
    fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut sum: usize = 0;
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let mut group = HashMap::new();

    contents.lines().for_each(|line| {
        for item in line
            .bytes()
            .filter_map(|i| Item::try_from(i).ok())
            .collect::<HashSet<Item>>()
        {
            let counter = group.entry(item).or_insert(0);
            *counter += 1;
            if *counter == 3 {
                sum += item.score();
                group.clear();
                break;
            }
        }
    });
    println!("{}", sum);
}
