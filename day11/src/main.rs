use std::mem;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, space0, space1, u64},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
#[derive(Debug)]
enum Value {
    Old,
    Number(u64),
}

impl Value {
    fn parse(input: &str) -> IResult<&str, Value> {
        let (input, value) = alt((
            map(tag("old"), |_| Value::Old),
            map(u64, |n: u64| Value::Number(n)),
        ))(input)?;
        Ok((input, value))
    }
}
#[derive(Debug)]
struct Operation {
    operation: char,
    value: Value,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Operation> {
        map(
            tuple((
                delimited(space0, alt((char('+'), char('*'))), space0),
                Value::parse,
            )),
            |(operation, value)| Operation { operation, value },
        )(input)
    }
}

#[derive(Debug)]
struct Monkey {
    number: u64,
    items: Vec<u64>,
    operation: Operation,
    devisable: u64,
    throw: (u64, u64),
    inspections: u64,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Monkey> {
        let (input, number) = parse_monkey_number(&input)?;
        let (input, items) = parse_starting_items(&input)?;
        let (input, operation) = parse_operation(&input)?;
        let (input, devisable) = parse_devisible(&input)?;
        let (input, throw) = parse_throw(&input)?;
        Ok((
            input,
            Monkey {
                number,
                items,
                operation,
                devisable,
                throw,
                inspections: 0,
            },
        ))
    }
    fn operation(&mut self, number: u64) -> u64 {
        match self.operation.operation {
            '+' => match self.operation.value {
                Value::Old => number + number,
                Value::Number(n) => number + n,
            },
            '*' => match self.operation.value {
                Value::Old => number * number,
                Value::Number(n) => number * n,
            },
            _ => unreachable!(),
        }
    }
    fn process_items(&mut self, divisor_product: u128) -> Vec<(usize, u64)> {
        self.inspections = self.inspections + self.items.len() as u64;
        let mut items = vec![];
        let old_items = mem::replace(&mut self.items, vec![]);
        old_items.iter().for_each(|number| {
            let mut new_number = number % divisor_product as u64;
            new_number = self.operation(new_number);
            if new_number % self.devisable == 0 {
                items.push((self.throw.0 as usize, new_number));
            } else {
                items.push((self.throw.1 as usize, new_number));
            }
        });
        return items;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (_, mut monkeys) = parse_monkeys(&input).unwrap();
    let divisor_product = monkeys
        .iter()
        .map(|m| m.devisable as u128)
        .product::<u128>();
    (0..10_000).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            monkeys[i]
                .process_items(divisor_product)
                .iter()
                .for_each(|(monkey_num, number)| {
                    monkeys[*monkey_num].items.push(*number);
                });
        });
    });
    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    monkeys
        .iter()
        .for_each(|m| println!("monkey {} {}", m.number, m.inspections));
    println!(
        "monkeybusiness = {}",
        monkeys[monkeys.len() - 1].inspections as u128
            * monkeys[monkeys.len() - 2].inspections as u128
    );
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    many1(terminated(Monkey::parse, opt(line_ending)))(input)
}

fn parse_monkey_number(input: &str) -> IResult<&str, u64> {
    delimited(tag("Monkey "), u64, tuple((char(':'), line_ending)))(input)
}
fn parse_list_item(input: &str) -> IResult<&str, u64> {
    preceded(space0, u64)(input)
}
fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) = preceded(
        tuple((space1, tag("Starting items:"))),
        terminated(many1(tuple((parse_list_item, opt(char(','))))), tag("\n")),
    )(input)?;
    Ok((input, numbers.into_iter().map(|(n, _)| n).collect()))
}
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = preceded(
        tuple((space1, tag("Operation: new = old"))),
        terminated(Operation::parse, tag("\n")),
    )(input)?;
    Ok((input, operation))
}

fn parse_devisible(input: &str) -> IResult<&str, u64> {
    delimited(
        tuple((space1, tag("Test: divisible by "))),
        u64,
        line_ending,
    )(input)
}
fn parse_throw_line(input: &str) -> IResult<&str, (bool, u64)> {
    tuple((
        map(
            delimited(
                tuple((space1, tag("If "))),
                alt((tag("true"), tag("false"))),
                tag(": "),
            ),
            |s| match s {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            },
        ),
        delimited(tag("throw to monkey "), u64, line_ending),
    ))(input)
}

fn parse_throw(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (_, first)) = parse_throw_line(input)?;
    let (input, (_, second)) = parse_throw_line(input)?;
    Ok((input, (first, second)))
}
