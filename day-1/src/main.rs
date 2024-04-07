use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct InvalidDigitCountError;

fn get_num_sum_from_str(input: &str) -> Result<u32, InvalidDigitCountError> {
    let mut nums = Vec::new();
    for character in input.chars() {
        if character.is_numeric() {
            nums.push(character.to_digit(10).unwrap());
        }
    }

    match nums.len() {
        1 => Ok(nums[0] + (10 * nums[0])),
        2.. => Ok(nums[nums.len() - 1] + (10 * nums[0])),
        _ => Err(InvalidDigitCountError),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_calibrations(filename: &str) -> u32 {
    let mut sum: u32 = 0;

    let Ok(lines) = read_lines(filename) else {
        panic!("Could not read from file {}", filename);
    };

    for line in lines.flatten() {
        if line.eq("quit") {
            break;
        }

        if let Ok(num) = get_num_sum_from_str(&line) {
            sum += num;
        }
    }

    sum
}

fn main() {
    let sum = parse_calibrations("./input.txt");

    println!("sum: {}", sum);
}
