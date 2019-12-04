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

fn populate(occupied: &mut HashSet<Coord>, point: Coord, intersect: &mut HashSet<Coord>) {
    dbg!(point);
    if !occupied.insert(point) {
        intersect.insert(point);
    }
}

#[test]
fn test_populate() {
    let mut occupied = HashSet::new();
    let mut intersect = HashSet::new();
    
    populate(&mut occupied, Coord::new(1, 2), &mut intersect);
    assert_eq!(1, occupied.len());
    assert_eq!(0, intersect.len());
    
    populate(&mut occupied, Coord::new(2, 2), &mut intersect);
    assert_eq!(2, occupied.len());
    assert_eq!(0, intersect.len());

    populate(&mut occupied, Coord::new(1, 2), &mut intersect);
    assert_eq!(2, occupied.len());
    assert_eq!(1, intersect.len());
}

fn populate_move(current: &Coord, mv: &Move, occupied: &mut HashSet<Coord>, intersect: &mut HashSet<Coord>) {
    match mv {
        Move::Up(amount) => {
            for i in current.y..(current.y + amount) {
                populate(occupied, Coord::new(current.x, i + 1), intersect);
            }
        },
        Move::Down(amount) => {
            for i in (current.y - amount)..current.y {
                populate(occupied, Coord::new(current.x, i), intersect);
            }
        },
        Move::Left(amount) => {
            for i in (current.x - amount)..current.x {
                populate(occupied, Coord::new(i, current.y), intersect);
            }
        },
        Move::Right(amount) => {
            for i in current.x..(current.x + amount) {
                populate(occupied, Coord::new(i + 1, current.y), intersect);
            }
        },
    }
}

#[test]
fn test_populate_move() {
    let mut occupied: HashSet<Coord> = HashSet::new();
    let mut intersect: HashSet<Coord> = HashSet::new();

    populate_move(&Coord::new(1, 1), &Move::Up(3), &mut occupied, &mut intersect);
    assert_eq!(3, occupied.len());
    assert!(occupied.contains(&Coord::new(1, 2)));
    assert!(occupied.contains(&Coord::new(1, 3)));
    assert!(occupied.contains(&Coord::new(1, 4)));

    occupied.clear();
    populate_move(&Coord::new(1, 1), &Move::Down(3), &mut occupied, &mut intersect);
    assert_eq!(3, occupied.len());
    assert!(occupied.contains(&Coord::new(1, 0)));
    assert!(occupied.contains(&Coord::new(1, -1)));
    assert!(occupied.contains(&Coord::new(1, -2)));

    occupied.clear();
    populate_move(&Coord::new(1, 1), &Move::Left(3), &mut occupied, &mut intersect);
    assert_eq!(3, occupied.len());
    assert!(occupied.contains(&Coord::new(0, 1)));
    assert!(occupied.contains(&Coord::new(-1, 1)));
    assert!(occupied.contains(&Coord::new(-2, 1)));

    occupied.clear();
    populate_move(&Coord::new(1, 1), &Move::Right(3), &mut occupied, &mut intersect);
    assert_eq!(3, occupied.len());
    assert!(occupied.contains(&Coord::new(2, 1)));
    assert!(occupied.contains(&Coord::new(3, 1)));
    assert!(occupied.contains(&Coord::new(4, 1)));
}

#[test]
fn test_intersect() {
    // TODO two moves that intersect
}

// TODO: shortest manhattan distance

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let mut f = BufReader::new(f);

    let mut line1 = String::from("");
    f.read_line(&mut line1).unwrap();
    let mut line2 = String::from("");
    f.read_line(&mut line2).unwrap();

    let first_moves = parse_moves(line1.split(',').map(|m| m.into()).collect::<Vec<String>>());
    let second_moves = parse_moves(line2.split(',').map(|m| m.into()).collect::<Vec<String>>());

    //let mut occupied = HashSet::new();

    Ok(())
}
