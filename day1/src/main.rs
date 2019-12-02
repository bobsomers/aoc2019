use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

fn calculate_fuel(mass: i32) -> i32 {
    let result = (mass / 3) - 2;
    if result <= 0 {
        0
    } else {
        result
    }
}

#[test]
fn fuel_calculation() {
    assert_eq!(2, calculate_fuel(12));
    assert_eq!(2, calculate_fuel(14));
    assert_eq!(654, calculate_fuel(1969));
    assert_eq!(33583, calculate_fuel(100756));
    assert_eq!(0, calculate_fuel(0));
    assert_eq!(0, calculate_fuel(-2));
}

fn calculate_total(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        fuel
    } else {
        fuel + calculate_total(fuel)
    }
}

#[test]
fn total_calculation() {
    assert_eq!(2, calculate_total(14));
    assert_eq!(966, calculate_total(1969));
    assert_eq!(50346, calculate_total(100756));
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let total_fuel: i32 = f.lines()
        .map(|line| calculate_total(i32::from_str(&line.unwrap()).unwrap()))
        .sum();
    println!("Total Fuel: {}", total_fuel);

    Ok(())
}
