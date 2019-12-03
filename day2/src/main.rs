use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

fn step(program: &mut Vec<i32>, pc: usize) -> bool {
    let opcode = program[pc];
    match opcode {
        1 => {
            let src1 = program[pc + 1] as usize;
            let src2 = program[pc + 2] as usize;
            let dest = program[pc + 3] as usize;
            program[dest] = program[src1] + program[src2];
        },
        2 => {
            let src1 = program[pc + 1] as usize;
            let src2 = program[pc + 2] as usize;
            let dest = program[pc + 3] as usize;
            program[dest] = program[src1] * program[src2];
        },
        99 => return true,
        _ => panic!("Unknown opcode {}!", opcode),
    }
    false
}

fn execute(program: &mut Vec<i32>) {
    let mut pc = 0;
    let mut done = false;
    while !done {
        done = step(program, pc);
        pc += 4;
    }
}

#[test]
fn example_programs() {
    let mut p1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    execute(&mut p1);
    assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], p1);

    let mut p2 = vec![1, 0, 0, 0, 99];
    execute(&mut p2);
    assert_eq!(vec![2, 0, 0, 0, 99], p2);

    let mut p3 = vec![2, 3, 0, 3, 99];
    execute(&mut p3);
    assert_eq!(vec![2, 3, 0, 6, 99], p3);

    let mut p4 = vec![2, 4, 4, 5, 99, 0];
    execute(&mut p4);
    assert_eq!(vec![2, 4, 4, 5, 99, 9801], p4);

    let mut p5 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    execute(&mut p5);
    assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], p5);
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // Load the program.
    let program: Vec<i32> = f.split(',' as u8)
        .map(|x| i32::from_str(&String::from_utf8(x.unwrap()).unwrap()).unwrap()).collect();

    // Search for the "noun" and "verb".
    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            execute(&mut memory);
            let output = memory[0];

            if output == 19690720 {
                println!("Noun = {}", noun);
                println!("Verb = {}", verb);
                println!("100 * Noun + Verb = {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
