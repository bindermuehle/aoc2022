use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
enum RoundResult {
    Win,
    Lose,
    Draw,
}
impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Invalid input")),
        }
    }
}

impl Move {
    fn beats(&self, m: &Move) -> bool {
        match (self, m) {
            (Self::Rock, Self::Scissors) => true,
            (Self::Paper, Self::Rock) => true,
            (Self::Scissors, Self::Paper) => true,
            _ => false,
        }
    }
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Round {
    player_move: Move,
    opponent_move: Move,
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 3 {
            return Err(color_eyre::eyre::eyre!("Invalid input"));
        }
        Ok(Self {
            player_move: chars[0].try_into()?,
            opponent_move: chars[2].try_into()?,
        })
    }
}

impl Round {
    fn result(&self) -> RoundResult {
        if self.player_move == self.opponent_move {
            return RoundResult::Draw;
        }
        if self.player_move.beats(&self.opponent_move) {
            return RoundResult::Win;
        }
        RoundResult::Lose
    }
    fn score(&self) -> i32 {
        let mut score = self.player_move.value();

        score += match self.result() {
            RoundResult::Win => 6,
            RoundResult::Lose => 3,
            RoundResult::Draw => 0,
        };
        return score;
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let sum: i32 = contents
        .lines()
        .filter_map(|l| l.parse::<Round>().ok())
        .map(|round| round.score())
        .sum();

    println!("Sum: {}", sum);
}

// Rock A Y
// Paper B X
// Scissors C Z
