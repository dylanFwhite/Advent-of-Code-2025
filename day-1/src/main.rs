use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut zero_count: u32 = 0;
    let mut dial: u32 = 50;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let (direction, number) = parse_instruction(&line);
            let (new_dial, zeros) = process_instruction(&dial, direction, &number);

            zero_count += zeros;
            dial = new_dial;

            println!("Line: {line} - Dial: {dial} - Zeros: {zeros} - Count: {zero_count}");
        }
    }
    println!("Password: {zero_count}");
}

fn process_instruction(dial: &u32, direction: &str, number: &u32) -> (u32, u32) {
    match direction {
        "L" => decrease_dial(dial, number),
        "R" => increase_dial(dial, number),
        _ => (9_999_999, 9_999_999),
    }
}

fn decrease_dial(dial: &u32, number: &u32) -> (u32, u32) {
    let effective_num = number % 100;
    let rotations = number / 100;

    if dial == &0 {
        (100 - effective_num, rotations)
    } else if &effective_num > dial {
        (100 - (effective_num - dial), rotations + 1)
    } else if &effective_num == dial {
        (0, rotations + 1)
    } else {
        (dial - effective_num, rotations)
    }
}

fn increase_dial(dial: &u32, number: &u32) -> (u32, u32) {
    let effective_num = number % 100;
    let rotations = number / 100;

    if (dial + effective_num) == 100 {
        (0, rotations + 1)
    } else if dial + effective_num > 100 {
        ((dial + effective_num) - 100, rotations + 1)
    } else {
        (dial + effective_num, rotations)
    }
}

fn parse_instruction(inst: &str) -> (&str, u32) {
    let re_dir = Regex::new("L|R").unwrap();
    let re_num = Regex::new("\\d+").unwrap();

    let dir = re_dir.find(inst).unwrap().as_str();
    let num_str = re_num.find(inst).unwrap().as_str().parse::<u32>().unwrap();

    (dir, num_str)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
