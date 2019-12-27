use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn parse_moves(strings: Vec<String>) -> Vec<Move> {
    let mut moves = vec![];
    for mut direction in strings {
        let amount = i32::from_str(&direction.split_off(1)).unwrap();
        match direction.as_str() {
            "U" => moves.push(Move::Up(amount)),
            "D" => moves.push(Move::Down(amount)),
            "L" => moves.push(Move::Left(amount)),
            "R" => moves.push(Move::Right(amount)),
            _ => panic!("Unknown direction {}", direction),
        }
    }
    return moves;
}

#[test]
fn test_parse_moves() {
    assert_eq!(vec![Move::Up(7), Move::Right(6), Move::Down(4), Move::Left(4)],
        parse_moves(vec!["U7".into(), "R6".into(), "D4".into(), "L4".into()]));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Marker {
    c: Coord,
    step: i32,
}

impl Marker {
    fn new(x: i32, y: i32, step: i32) -> Marker {
        Marker { c: Coord::new(x, y), step }
    }
}

fn populate_move(current: &Marker, mv: &Move, occupied: &mut HashMap<Coord, i32>) -> Marker {
    let cur = current.c;
    let mut step = current.step;
    match mv {
        Move::Up(amount) => {
            for y in (cur.y + 1)..(cur.y + amount + 1) {
                step += 1;
                let c = Coord::new(cur.x, y);
                if !occupied.contains_key(&c) {
                    occupied.insert(c, step);
                }
            }
        },

        Move::Down(amount) => {
            for y in ((cur.y - amount)..cur.y).rev() {
                step += 1;
                let c = Coord::new(cur.x, y);
                if !occupied.contains_key(&c) {
                    occupied.insert(c, step);
                }
            }

        },

        Move::Left(amount) => {
            for x in ((cur.x - amount)..cur.x).rev() {
                step += 1;
                let c = Coord::new(x, cur.y);
                if !occupied.contains_key(&c) {
                    occupied.insert(c, step);
                }
            }

        },

        Move::Right(amount) => {
            for x in (cur.x + 1)..(cur.x + amount + 1) {
                step += 1;
                let c = Coord::new(x, cur.y);
                if !occupied.contains_key(&c) {
                    occupied.insert(c, step);
                }
            }
        }
    }

    // Return new cursor position.
    match mv {
        Move::Up(amount) => Marker::new(cur.x, cur.y + amount, current.step + amount),
        Move::Down(amount) => Marker::new(cur.x, cur.y - amount, current.step + amount),
        Move::Left(amount) => Marker::new(cur.x - amount, cur.y, current.step + amount),
        Move::Right(amount) => Marker::new(cur.x + amount, cur.y, current.step + amount),
    }
}

#[test]
fn test_populate_move() {
    let mut occupied: HashMap<Coord, i32> = HashMap::new();
    let cursor = populate_move(&Marker::new(1, 1, 1), &Move::Up(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Marker::new(1, 4, 4), cursor);
    assert_eq!(Some(&2), occupied.get(&Coord::new(1, 2)));
    assert_eq!(Some(&3), occupied.get(&Coord::new(1, 3)));
    assert_eq!(Some(&4), occupied.get(&Coord::new(1, 4)));

    let mut occupied: HashMap<Coord, i32> = HashMap::new();
    let cursor = populate_move(&Marker::new(1, 1, 1), &Move::Down(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Marker::new(1, -2, 4), cursor);
    assert_eq!(Some(&2), occupied.get(&Coord::new(1, 0)));
    assert_eq!(Some(&3), occupied.get(&Coord::new(1, -1)));
    assert_eq!(Some(&4), occupied.get(&Coord::new(1, -2)));

    let mut occupied: HashMap<Coord, i32> = HashMap::new();
    let cursor = populate_move(&Marker::new(1, 1, 1), &Move::Left(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Marker::new(-2, 1, 4), cursor);
    assert_eq!(Some(&2), occupied.get(&Coord::new(0, 1)));
    assert_eq!(Some(&3), occupied.get(&Coord::new(-1, 1)));
    assert_eq!(Some(&4), occupied.get(&Coord::new(-2, 1)));

    let mut occupied: HashMap<Coord, i32> = HashMap::new();
    let cursor = populate_move(&Marker::new(1, 1, 1), &Move::Right(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Marker::new(4, 1, 4), cursor);
    assert_eq!(Some(&2), occupied.get(&Coord::new(2, 1)));
    assert_eq!(Some(&3), occupied.get(&Coord::new(3, 1)));
    assert_eq!(Some(&4), occupied.get(&Coord::new(4, 1)));
}

fn populate_wire(moves: Vec<Move>) -> HashMap<Coord, i32> {
    let mut cursor = Marker::new(0, 0, 0);
    let mut occupied = HashMap::new();
    for mv in moves {
        cursor = populate_move(&cursor, &mv, &mut occupied);
    }
    occupied
}

#[test]
fn test_populate_wire() {
    let wire = vec![Move::Right(8), Move::Up(5), Move::Left(5), Move::Down(3)];
    let occupied = populate_wire(wire);
    assert_eq!(21, occupied.len());
    assert_eq!(Some(&1), occupied.get(&Coord::new(1, 0)));
    assert_eq!(Some(&2), occupied.get(&Coord::new(2, 0)));
    assert_eq!(Some(&3), occupied.get(&Coord::new(3, 0)));
    assert_eq!(Some(&4), occupied.get(&Coord::new(4, 0)));
    assert_eq!(Some(&5), occupied.get(&Coord::new(5, 0)));
    assert_eq!(Some(&6), occupied.get(&Coord::new(6, 0)));
    assert_eq!(Some(&7), occupied.get(&Coord::new(7, 0)));
    assert_eq!(Some(&8), occupied.get(&Coord::new(8, 0)));
    assert_eq!(Some(&9), occupied.get(&Coord::new(8, 1)));
    assert_eq!(Some(&10), occupied.get(&Coord::new(8, 2)));
    assert_eq!(Some(&11), occupied.get(&Coord::new(8, 3)));
    assert_eq!(Some(&12), occupied.get(&Coord::new(8, 4)));
    assert_eq!(Some(&13), occupied.get(&Coord::new(8, 5)));
    assert_eq!(Some(&14), occupied.get(&Coord::new(7, 5)));
    assert_eq!(Some(&15), occupied.get(&Coord::new(6, 5)));
    assert_eq!(Some(&16), occupied.get(&Coord::new(5, 5)));
    assert_eq!(Some(&17), occupied.get(&Coord::new(4, 5)));
    assert_eq!(Some(&18), occupied.get(&Coord::new(3, 5)));
    assert_eq!(Some(&19), occupied.get(&Coord::new(3, 4)));
    assert_eq!(Some(&20), occupied.get(&Coord::new(3, 3)));
    assert_eq!(Some(&21), occupied.get(&Coord::new(3, 2)));
}

fn find_intersections(a: &HashMap<Coord, i32>, b: &HashMap<Coord, i32>) -> HashMap<Coord, i32> {
    let mut intersections = HashMap::new();
    for (c, c_steps) in a {
        if b.contains_key(c) {
            intersections.insert(*c, c_steps + b.get(c).unwrap());
        }
    }
    intersections
}

#[test]
fn test_find_intersections() {
    let a = populate_wire(vec![Move::Right(8), Move::Up(5), Move::Left(5), Move::Down(3)]);
    let b = populate_wire(vec![Move::Up(7), Move::Right(6), Move::Down(4), Move::Left(4)]);
    let intersections = find_intersections(&a, &b);
    assert_eq!(2, intersections.len());
    assert_eq!(Some(&40), intersections.get(&Coord::new(3, 3)));
    assert_eq!(Some(&30), intersections.get(&Coord::new(6, 5)));
}

fn shortest_steps(a: &str, b: &str) -> i32 {
    let a_moves = parse_moves(a.trim().split(',').map(|m| m.into()).collect::<Vec<String>>());
    let a_occupied = populate_wire(a_moves);
    let b_moves = parse_moves(b.trim().split(',').map(|m| m.into()).collect::<Vec<String>>());
    let b_occupied = populate_wire(b_moves);
    let intersections = find_intersections(&a_occupied, &b_occupied);
    let mut distances = intersections.values().collect::<Vec<&i32>>();
    distances.sort();
    *distances[0]
}

#[test]
fn test_shortest_steps() {
    assert_eq!(30, shortest_steps("R8,U5,L5,D3", "U7,R6,D4,L4"));
    assert_eq!(610, shortest_steps("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(410, shortest_steps("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let mut f = BufReader::new(f);

    // Load lines with moves.
    let mut line1 = String::from("");
    f.read_line(&mut line1).unwrap();
    let mut line2 = String::from("");
    f.read_line(&mut line2).unwrap();

    // Do it!
    let shortest = shortest_steps(&line1, &line2);

    // Get the shortest number of combined steps.
    println!("Shortest combined steps to intersection: {}", shortest);

    Ok(())
}
