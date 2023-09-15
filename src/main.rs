use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Command {
    Add(i32),
    Noop,
}

const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

struct CPU {
    register: i32,
    program_counter: usize,
}
impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            program_counter: 0,
        }
    }
    fn execute_command(&mut self, command: &Command) {
        match command {
            Command::Add(number) => {
                self.increase_counter(2);
                self.register += number;
            }
            Command::Noop => self.increase_counter(1),
        }
    }
    fn increase_counter(&mut self, amount: usize) {
        (1..=amount).for_each(|_| {
            self.print();
            self.program_counter += 1;
        });
    }
    fn print(&self) {
        let position = &self.program_counter % 40;
        if position == 0 && self.program_counter != 0 {
            print!("\n");
        }
        if position as i32 >= self.register - 1 && position as i32 <= self.register + 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    let content = std::fs::read_to_string("input.txt").unwrap();
    let (_, commands) = parse_commands(&content).unwrap();
    commands.iter().for_each(|command| {
        cpu.execute_command(command);
    });
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = many1(parse_command)(input)?;
    Ok((input, commands))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, (command, _)) = tuple((alt((parse_add, parse_noop)), tag("\n")))(input)?;
    return Ok((input, command));
}
fn parse_add(i: &str) -> IResult<&str, Command> {
    map(
        preceded(tag("addx "), nom::character::complete::i32),
        Command::Add,
    )(i)
}

fn parse_noop(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Command::Noop))
}
