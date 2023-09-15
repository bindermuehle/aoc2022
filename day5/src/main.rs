use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, line_ending, newline, space1},
    combinator::{eof, opt},
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cargo(char);

#[derive(Debug, PartialEq, Clone)]
struct Ship {
    cargo: Vec<Vec<Cargo>>,
}

impl Ship {
    fn from_rows(mut rows: Vec<Vec<Option<Cargo>>>) -> Ship {
        let mut cargo: Vec<Vec<Cargo>> = vec![vec![]; rows[0].len()];
        if rows.len() == 0 {
            return Ship { cargo };
        }
        rows.reverse();

        rows.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(index, container)| {
                if cargo.get(index).is_none() {
                    cargo.insert(index, vec![]);
                }
                if let Some(container) = container {
                    cargo[index].push(container.clone());
                }
            })
        });
        Ship { cargo: cargo }
    }
    fn apply(&mut self, instruction: Instruction) -> () {
        for c in (0..instruction.quantity)
            .map(|_| self.cargo[instruction.from - 1].pop().unwrap())
            .collect::<Vec<Cargo>>()
            .into_iter()
            .rev()
        {
            self.cargo[instruction.to - 1].push(c);
        }
    }
    fn to_string(&self) -> String {
        self.cargo
            .iter()
            .map(|row| {
                if let Some(container) = row.last() {
                    return Some(container.0);
                }
                None
            })
            .filter_map(|x| x)
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_ship_with_instructions(input: &str) -> IResult<&str, (Ship, Vec<Instruction>)> {
    let (rest, ship) = parse_ship(input)?;
    let (rest, _) = many1(line_ending)(rest)?;
    let (rest, instructions) = many1(parse_instruction)(rest)?;
    Ok((rest, (ship, instructions)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("move")(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, quantity) = digit1(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, _) = tag("from")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, from) = digit1(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, _) = tag("to")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, to) = digit1(rest)?;
    let (rest, _) = alt((line_ending, eof))(rest)?;
    Ok((
        rest,
        Instruction {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            quantity: quantity.parse().unwrap(),
        },
    ))
}

fn parse_empty_spot(input: &str) -> IResult<&str, Option<Cargo>> {
    match tuple((char(' '), char(' '), char(' ')))(input) {
        Ok((rest, _)) => Ok((rest, None)),
        Err(e) => Err(e),
    }
}
fn parse_cargo(input: &str) -> IResult<&str, Option<Cargo>> {
    let mut parser = delimited(char('['), anychar, char(']'));

    let result = parser(input);
    match result {
        Ok((rest, cargo)) => Ok((rest, Some(Cargo(cargo)))),
        Err(e) => Err(e),
    }
}
fn parse_cargo_line(input: &str) -> IResult<&str, Vec<Option<Cargo>>> {
    match many0(tuple((
        alt((parse_cargo, parse_empty_spot)),
        opt(char(' ')),
    )))(input)
    {
        Ok((rest, cargo)) => match newline(rest) {
            Ok((rest, _)) => {
                let cargo = cargo.iter().copied().map(|(cargo, _)| cargo).collect();
                Ok((rest, cargo))
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn parse_ship_hull(input: &str) -> IResult<&str, ()> {
    match many1(alt((space1, digit1)))(input) {
        Ok((rest, _)) => {
            let (rest, _) = newline(rest)?;
            Ok((rest, ()))
        }
        Err(e) => Err(e),
    }
}

fn parse_ship(input: &str) -> IResult<&str, Ship> {
    match tuple((many1(parse_cargo_line), (parse_ship_hull)))(input) {
        Ok((rest, (cargo, _))) => Ok((rest, Ship::from_rows(cargo))),
        Err(e) => Err(e),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (_, (mut ship, instruction)) = parse_ship_with_instructions(&input).unwrap();

    for instruction in instruction {
        ship.apply(instruction);
    }
    println!("{:?}", ship.to_string());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(parse_cargo("[D]"), Ok(("", Some(Cargo('D')))));
    }
    #[test]
    fn test_empty_spot() {
        assert_eq!(parse_empty_spot("   "), Ok(("", None)));
    }
    #[test]
    fn test_cargo_line() {
        assert_eq!(
            parse_cargo_line("[D]     [C]\n"),
            Ok(("", vec![Some(Cargo('D')), None, Some(Cargo('C'))]))
        );
        assert_eq!(
            parse_cargo_line("    [D]    \n"),
            Ok(("", vec![None, Some(Cargo('D')), None]))
        );
        assert_eq!(
            parse_cargo_line("[N] [C]    \n"),
            Ok(("", vec![Some(Cargo('N')), Some(Cargo('C')), None]))
        );
    }
    #[test]
    fn check_line_with_numbers() {
        assert_eq!(parse_ship_hull(" 1   2   3 \n"), Ok(("", ())));
    }

    #[test]
    fn test_parse_ship() {
        //assert_eq!(
        //     parse_ship("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n"),
        //     Ok((
        //         "",
        //         Ship {
        //             cargo: vec![
        //                 vec![None, Some(Cargo('D')), None],
        //                 vec![Some(Cargo('N')), Some(Cargo('C')), None],
        //                 vec![Some(Cargo('Z')), Some(Cargo('M')), Some(Cargo('P'))]
        //             ]
        //         }
        //     ))
        // );
    }
    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("move 1 from 2 to 3\n"),
            Ok((
                "",
                Instruction {
                    from: 2,
                    to: 3,
                    quantity: 1
                }
            ))
        );
        assert_eq!(
            parse_instruction("move 1 from 2 to 3"),
            Ok((
                "",
                Instruction {
                    from: 2,
                    to: 3,
                    quantity: 1
                }
            ))
        );
    }
}
