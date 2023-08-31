use nom::{
    branch::alt,
    character::complete::{anychar, char, digit1, newline, space1},
    combinator::opt,
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cargo(char);

#[derive(Debug, PartialEq, Clone)]
struct Ship {
    cargo: Vec<Vec<Option<Cargo>>>,
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
        Ok((rest, (cargo, _))) => Ok((rest, Ship { cargo })),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("Hello, world!");
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
        assert_eq!(
            parse_ship("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n"),
            Ok((
                "",
                Ship {
                    cargo: vec![
                        vec![None, Some(Cargo('D')), None],
                        vec![Some(Cargo('N')), Some(Cargo('C')), None],
                        vec![Some(Cargo('Z')), Some(Cargo('M')), Some(Cargo('P'))]
                    ]
                }
            ))
        );
    }
}
