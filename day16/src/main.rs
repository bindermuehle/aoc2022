use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
#[derive(Debug, Clone)]
struct Room {
    name: String,
    pressure: u64,
    tunnels: Vec<String>,
    connections: HashMap<String, u64>,
}
impl Room {
    fn parse(input: &str) -> IResult<&str, Room> {
        let (input, (name, valve, tunnels)) = tuple((
            delimited(tag("Valve "), alpha1, tag(" has flow rate=")),
            terminated(u64, tag(";")),
            preceded(
                alt((
                    tag(" tunnels lead to valves "),
                    tag(" tunnel leads to valve "),
                )),
                separated_list1(tag(", "), map(alpha1, |s: &str| s.to_string())),
            ),
        ))(input)?;
        Ok((
            input,
            Room {
                name: name.to_string(),
                pressure: valve,
                tunnels,
                connections: HashMap::new(),
            },
        ))
    }
}
#[derive(Clone, Debug)]
struct State {
    released_valves: HashSet<String>,
    released_pressure: u64,
    time_left: u64,
    current: String,
}

impl State {
    fn new(current: String) -> State {
        State {
            released_valves: HashSet::new(),
            released_pressure: 0,
            time_left: 30,
            current,
        }
    }
    fn walk(&mut self, room: &Room) {
        self.time_left -= room.connections.get(&self.current).unwrap() as &u64;
        self.current = room.name.clone();
    }

    fn release(&mut self, pressure: u64, room: String) {
        self.time_left -= 1;
        self.released_valves.insert(room.clone());
        self.released_pressure += pressure;
    }
}
struct Cave {
    rooms: HashMap<String, Room>,
}
impl Cave {
    fn find_max_pressure(&self, state: State) -> u64 {
        let room = self.rooms.get(&state.current).unwrap();

        let max = room
            .connections
            .iter()
            .map(|(name, distance)| {
                if state.released_valves.contains(name)
                    || self.rooms.get(name).unwrap().pressure == 0
                    || state.time_left < distance + 1
                {
                    state.released_pressure
                } else {
                    let next = self.rooms.get(name).unwrap();
                    let mut new_state = state.clone();
                    let pressure = next.pressure * (state.time_left - distance - 1);
                    new_state.walk(next);
                    new_state.release(pressure, name.clone());
                    return self.find_max_pressure(new_state);
                }
            })
            .max();
        if let Some(max) = max {
            return max;
        } else {
            return state.released_pressure;
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut rooms: HashMap<String, Room> = input
        .lines()
        .map(|line| {
            let room = Room::parse(line).unwrap().1;
            (room.name.clone(), room)
        })
        .collect();
    populate_network(&mut rooms);
    let state = State::new(rooms.get("AA").unwrap().name.clone());
    let cave = Cave { rooms };
    let num = cave.find_max_pressure(state);
    println!("{}", num);
}
fn populate_network(rooms: &mut HashMap<String, Room>) {
    let lookup = rooms.clone();
    for (name, room) in rooms.into_iter() {
        let mut connections: HashMap<String, u64> = HashMap::new();
        build_connections(&mut connections, name.clone(), &lookup, 0);
        room.connections = connections;
    }
}

fn build_connections(
    connections: &mut HashMap<String, u64>,
    current: String,
    rooms: &HashMap<String, Room>,
    mut distance: u64,
) {
    let tunnels = rooms.get(&current).unwrap().tunnels.clone();
    distance = distance + 1;
    tunnels.iter().for_each(|tunnel| {
        if !connections.contains_key(tunnel) || connections[tunnel] > distance {
            connections.insert(tunnel.clone(), distance);
            build_connections(connections, tunnel.clone(), rooms, distance);
        }
    })
}
