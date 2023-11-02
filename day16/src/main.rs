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
    elefant_current: String,
    elefant_time_left: u64,
}

impl State {
    fn new(current: String, elefant_current: String) -> State {
        State {
            released_valves: HashSet::new(),
            released_pressure: 0,
            time_left: 26,
            elefant_time_left: 26,
            current,
            elefant_current,
        }
    }
    fn walk(&mut self, room: &Room) {
        self.time_left -= room.connections.get(&self.current).unwrap() as &u64;
        self.current = room.name.clone();
    }
    fn elefant_walk(&mut self, room: &Room) {
        self.elefant_time_left -= room.connections.get(&self.elefant_current).unwrap() as &u64;
        self.elefant_current = room.name.clone();
    }
    fn elefant_release(&mut self, pressure: u64, room: String) {
        self.elefant_time_left -= 1;
        self.released_valves.insert(room.clone());
        self.released_pressure += pressure;
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
        let connections: Vec<(String, u64)> = room
            .connections
            .clone()
            .into_iter()
            .filter(|(name, distance)| {
                !state.released_valves.contains(name)
                    && self.rooms.get(name).unwrap().pressure != 0
                    && state.time_left > distance + 1
            })
            .collect();
        let elefant_room = self.rooms.get(&state.elefant_current).unwrap();
        let elefant_connections: Vec<(String, u64)> = elefant_room
            .connections
            .clone()
            .into_iter()
            .filter(|(name, distance)| {
                !state.released_valves.contains(name)
                    && self.rooms.get(name).unwrap().pressure != 0
                    && state.elefant_time_left > distance + 1
            })
            .collect();
        connections
            .iter()
            .map(|(name, distance)| {
                elefant_connections
                    .iter()
                    .filter_map(|(elefant_name, elefant_distance)| {
                        if name == elefant_name {
                            return None;
                        }

                        let mut new_state = state.clone();

                        if state.time_left > distance + 1 {
                            let next = self.rooms.get(name).unwrap();
                            let pressure = next.pressure * (state.time_left - distance - 1);
                            new_state.walk(next);
                            new_state.release(pressure, name.clone());
                        }
                        if state.elefant_time_left > elefant_distance + 1 {
                            let next = self.rooms.get(elefant_name).unwrap();
                            let pressure =
                                next.pressure * (state.elefant_time_left - elefant_distance - 1);
                            new_state.elefant_walk(next);
                            new_state.elefant_release(pressure, elefant_name.clone());
                        }
                        Some(self.find_max_pressure(new_state))
                    })
                    .max()
                    .unwrap_or(state.released_pressure)
            })
            .max()
            .unwrap_or(state.released_pressure)
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
    let state = State::new("AA".to_string(), "AA".to_string());
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
