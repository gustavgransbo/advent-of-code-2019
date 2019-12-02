use std::io::{self, BufRead};
use std::convert::TryInto;

fn run_until_complete(mut integer_program: Vec<u32>, replace_one: u32, replace_two: u32) -> Result<u32, &'static str> {
    integer_program[1] = replace_one;
    integer_program[2] = replace_two;

    for i in (0..integer_program.len()).step_by(4) {

        let op = integer_program[i];
        if op == 99 {break}

        let left_idx: usize = integer_program[i+1].try_into().unwrap();
        let right_idx: usize = integer_program[i+2].try_into().unwrap();
        let target_idx: usize = integer_program[i+3].try_into().unwrap();
        match op {
            1 => integer_program[target_idx] = integer_program[left_idx] + integer_program[right_idx],
            2 => integer_program[target_idx] = integer_program[left_idx] * integer_program[right_idx],
            _ => return Err("Invalid op-code")
        }
    }
    Ok(integer_program[0])
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let integer_program: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();

    println!("Answer part 1: {}", run_until_complete(integer_program.to_vec(), 12, 2).unwrap());

    for a in 0..99 {
        for b in 0..99 {
            match run_until_complete(integer_program.to_vec(), a, b) {
                Ok(19690720) => {println!("Answer part 2: {}", 100 * a + b); break},
                _ => continue
            }
        }
    }
}
