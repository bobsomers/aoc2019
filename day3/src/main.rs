use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashSet;

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

fn populate_move(current: &Coord, mv: &Move, occupied: &mut HashSet<Coord>) -> Coord {
    let mut x0 = current.x;
    let mut x1 = current.x + 1;
    let mut y0 = current.y;
    let mut y1 = current.y + 1;

    match mv {
        Move::Up(amount) => y1 += amount,
        Move::Down(amount) => y0 -= amount,
        Move::Left(amount) => x0 -= amount,
        Move::Right(amount) => x1 += amount,
    }

    for x in x0..x1 {
        for y in y0..y1 {
            if x == y {
                continue;
            }
            occupied.insert(Coord::new(x, y));
        }
    }

    // Return new cursor position.
    match mv {
        Move::Up(_) => Coord::new(x0, y1 - 1),
        Move::Down(_) => Coord::new(x0, y0),
        Move::Left(_) => Coord::new(x0, y0),
        Move::Right(_) => Coord::new(x1 - 1, y0),
    }
}

#[test]
fn test_populate_move() {
    let mut occupied: HashSet<Coord> = HashSet::new();
    let cursor = populate_move(&Coord::new(1, 1), &Move::Up(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Coord::new(1, 4), cursor);
    assert!(occupied.contains(&Coord::new(1, 2)));
    assert!(occupied.contains(&Coord::new(1, 3)));
    assert!(occupied.contains(&Coord::new(1, 4)));

    let mut occupied: HashSet<Coord> = HashSet::new();
    let cursor = populate_move(&Coord::new(1, 1), &Move::Down(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Coord::new(1, -2), cursor);
    assert!(occupied.contains(&Coord::new(1, 0)));
    assert!(occupied.contains(&Coord::new(1, -1)));
    assert!(occupied.contains(&Coord::new(1, -2)));

    let mut occupied: HashSet<Coord> = HashSet::new();
    let cursor = populate_move(&Coord::new(1, 1), &Move::Left(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Coord::new(-2, 1), cursor);
    assert!(occupied.contains(&Coord::new(0, 1)));
    assert!(occupied.contains(&Coord::new(-1, 1)));
    assert!(occupied.contains(&Coord::new(-2, 1)));

    occupied.clear();
    let cursor = populate_move(&Coord::new(1, 1), &Move::Right(3), &mut occupied);
    assert_eq!(3, occupied.len());
    assert_eq!(Coord::new(4, 1), cursor);
    assert!(occupied.contains(&Coord::new(2, 1)));
    assert!(occupied.contains(&Coord::new(3, 1)));
    assert!(occupied.contains(&Coord::new(4, 1)));
}

fn populate_wire(moves: Vec<Move>) -> HashSet<Coord> {
    let mut cursor = Coord::new(0, 0);
    let mut occupied = HashSet::new();
    for mv in moves {
        cursor = populate_move(&cursor, &mv, &mut occupied);
    }
    occupied
}

#[test]
fn populate_wire_test() {
    let wire = vec![Move::Right(8), Move::Up(5), Move::Left(5), Move::Down(3)];
    let occupied = populate_wire(wire);
    //assert_eq!(21, occupied.len());
    assert!(occupied.contains(&Coord::new(1, 0)));
    assert!(occupied.contains(&Coord::new(2, 0)));
    assert!(occupied.contains(&Coord::new(3, 0)));
    assert!(occupied.contains(&Coord::new(4, 0)));
    assert!(occupied.contains(&Coord::new(5, 0)));
    assert!(occupied.contains(&Coord::new(6, 0)));
    assert!(occupied.contains(&Coord::new(7, 0)));
    assert!(occupied.contains(&Coord::new(8, 0)));
    assert!(occupied.contains(&Coord::new(8, 1)));
    assert!(occupied.contains(&Coord::new(8, 2)));
    assert!(occupied.contains(&Coord::new(8, 3)));
    assert!(occupied.contains(&Coord::new(8, 4)));
    assert!(occupied.contains(&Coord::new(8, 5)));
    assert!(occupied.contains(&Coord::new(7, 5)));
    assert!(occupied.contains(&Coord::new(6, 5)));
    assert!(occupied.contains(&Coord::new(5, 5)));
    assert!(occupied.contains(&Coord::new(4, 5)));
    assert!(occupied.contains(&Coord::new(3, 5)));
    assert!(occupied.contains(&Coord::new(3, 4)));
    assert!(occupied.contains(&Coord::new(3, 3)));
    assert!(occupied.contains(&Coord::new(3, 2)));
}

#[test]
fn test_intersect() {
    // TODO two moves that intersect
}

// TODO: shortest manhattan distance

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let mut f = BufReader::new(f);

    // Load parse, and populate first wire moves.
    let mut line1 = String::from("");
    f.read_line(&mut line1).unwrap();
    let first_moves = parse_moves(line1.split(',').map(|m| m.into()).collect::<Vec<String>>());
    let _first_occupied = populate_wire(first_moves);

    // Load and parse second wire moves.
    let mut line2 = String::from("");
    f.read_line(&mut line2).unwrap();
    let _second_moves = parse_moves(line2.split(',').map(|m| m.into()).collect::<Vec<String>>());

    //let mut occupied = HashSet::new();

    Ok(())
}
