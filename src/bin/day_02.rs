use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct SubmarineState {
    depth: i32,
    position: i32,
    aim: i32,
}

impl SubmarineState {
    fn new() -> Self {
        Self {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let distance = tokens.get(1).ok_or("missing distance")?.parse::<i32>()?;
        match *tokens.get(0).ok_or("missing direction")? {
            "forward" => Ok(Command::Forward(distance)),
            "up" => Ok(Command::Up(distance)),
            "down" => Ok(Command::Down(distance)),
            _ => Err("unknown error")?,
        }
    }
}

impl Command {
    fn execute_on_point(&self, p: &Point) -> Point {
        match &self {
            Command::Forward(d) => Point { x: p.x + d, y: p.y },
            Command::Up(d) => Point { x: p.x, y: p.y - d },
            Command::Down(d) => Point { x: p.x, y: p.y + d },
        }
    }

    fn execute_on_state(&self, s: &SubmarineState) -> SubmarineState {
        match self {
            Command::Forward(d) => SubmarineState {
                depth: s.depth + s.aim * d,
                position: s.position + d,
                aim: s.aim,
            },
            Command::Up(d) => SubmarineState {
                depth: s.depth,
                position: s.position,
                aim: s.aim - d,
            },
            Command::Down(d) => SubmarineState {
                depth: s.depth,
                position: s.position,
                aim: s.aim + d,
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(args.get(1).expect("missing INPUT argument"));

    let file = File::open(input_path).expect("failed to open INPUT file");
    let reader = BufReader::new(file);

    let commands: Vec<Command> = reader
        .lines()
        .map(|x| {
            x.expect("failed to read line")
                .parse::<Command>()
                .expect("failed to parse command")
        })
        .collect();
    let final_position = commands
        .iter()
        .fold(Point { x: 0, y: 0 }, |p, c| c.execute_on_point(&p));

    println!("{:?}", final_position);
    println!("{}", final_position.x * final_position.y);

    let final_state = commands
        .iter()
        .fold(SubmarineState::new(), |s, c| c.execute_on_state(&s));
    println!("{:?}", final_state);
    println!("{}", final_state.depth * final_state.position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_commands() {
        let commands = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let mut commands_itr = commands.into_iter();
        assert_eq!(
            commands_itr
                .next()
                .unwrap()
                .execute_on_point(&Point { x: 0, y: 0 }),
            Point { x: 5, y: 0 }
        );
        assert_eq!(
            commands_itr
                .next()
                .unwrap()
                .execute_on_point(&Point { x: 5, y: 0 }),
            Point { x: 5, y: 5 }
        );
        assert_eq!(
            commands_itr
                .next()
                .unwrap()
                .execute_on_point(&Point { x: 5, y: 5 }),
            Point { x: 13, y: 5 }
        );
        assert_eq!(
            commands_itr
                .next()
                .unwrap()
                .execute_on_point(&Point { x: 13, y: 5 }),
            Point { x: 13, y: 2 }
        );
    }
}
