use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

fn step(program: &mut Vec<i32>, pc: usize) -> (bool, usize) {
    let opcode = program[pc];
    match opcode {
        1 => {
            let src1 = program[pc + 1] as usize;
            let src2 = program[pc + 2] as usize;
            let dest = program[pc + 3] as usize;
            program[dest] = program[src1] + program[src2];
            (false, 4)
        },
        2 => {
            let src1 = program[pc + 1] as usize;
            let src2 = program[pc + 2] as usize;
            let dest = program[pc + 3] as usize;
            program[dest] = program[src1] * program[src2];
            (false, 4)
        },
        3 => {
            let dest = program[pc + 1] as usize;
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let value = i32::from_str(input.trim()).unwrap();
            program[dest] = value;
            (false, 2)
        },
        4 => {
            let address = program[pc + 1] as usize;
            println!("{}", program[address]);
            (false, 2)
        },
        99 => (true, 0),
        _ => panic!("Unknown opcode {}!", opcode),
    }
}

fn execute(program: &mut Vec<i32>) {
    let mut pc = 0;
    let mut done = false;
    while !done {
        let (d, step_by) = step(program, pc);
        done = d;
        pc += step_by;
    }
}

#[test]
fn example_programs() {
    let mut p1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    execute(&mut p1);
    assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], p1);
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // Load the program.
//    let program: Vec<i32> = f.split(',' as u8)
//        .map(|x| i32::from_str(&String::from_utf8(x.unwrap()).unwrap()).unwrap()).collect();

    // Search for the "noun" and "verb".
//    'outer: for noun in 0..99 {
//        for verb in 0..99 {
//            let mut memory = program.clone();
//            memory[1] = noun;
//            memory[2] = verb;
//            execute(&mut memory);
//            let output = memory[0];
//
//            if output == 19690720 {
//                println!("Noun = {}", noun);
//                println!("Verb = {}", verb);
//                println!("100 * Noun + Verb = {}", 100 * noun + verb);
//                break 'outer;
//            }
//        }
//    }

    let mut program = vec![3, 0, 4, 0, 99];
    execute(&mut program);

    Ok(())
}
