#[derive(Debug)]
struct Assignment {
    fist: usize,
    last: usize,
}
struct Group {
    first: Assignment,
    second: Assignment,
}
impl TryFrom<&str> for Group {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (first, second) = value
            .split_once(',')
            .ok_or_else(|| color_eyre::eyre::eyre!("Invalid input"))?;
        let first = Assignment::try_from(first)?;
        let second = Assignment::try_from(second)?;
        return Ok(Self { first, second });
    }
}
impl Group {
    fn overlaps(&self) -> bool {
        return self.first.contains(&self.second) || self.second.contains(&self.first);
    }
}

impl TryFrom<&str> for Assignment {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (first, second) = value
            .split_once('-')
            .ok_or_else(|| color_eyre::eyre::eyre!("Invalid input"))?;
        return Ok(Self {
            fist: first.parse()?,
            last: second.parse()?,
        });
    }
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        return self.fist <= other.fist && self.last >= other.last;
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let sum = contents
        .lines()
        .map(|l| {
            let group = Group::try_from(l).unwrap();

            if group.overlaps() {
                return 1;
            }
            return 0;
        })
        .sum::<usize>();
    println!("{:?}", sum)
}
