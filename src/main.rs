use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
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

    contents.lines().for_each(|line| {
        let (first, second) = line.split_at(line.len() / 2);
        let first = first
            .bytes()
            .filter_map(|i| Item::try_from(i).ok())
            .collect::<HashSet<Item>>();
        let second = second
            .bytes()
            .filter_map(|i| Item::try_from(i).ok())
            .collect::<HashSet<Item>>();
        first.into_iter().for_each(|x| {
            if second.contains(&x) {
                sum += x.score();
            }
        });
    });
    println!("{}", sum);
}
