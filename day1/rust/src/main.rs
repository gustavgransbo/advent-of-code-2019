use std::io::{self, BufRead};

fn fuel_requirement_of_fuel(fuel: i32) -> i32 {
    // Recursively calculate the fuel requirement of fuel
    let fuel_required = fuel / 3 - 2;
    if fuel_required <= 0 {return 0};
    return fuel_required + fuel_requirement_of_fuel(fuel_required);
}

fn main() {
    let mut total_fuel_needed = 0;
    let mut total_fuel_needed_for_fuel = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // The problem input is always well formed, so we just unwrap
        let weight = line.unwrap().parse::<i32>().unwrap(); 
        let fuel_needed = (weight / 3) - 2;
        total_fuel_needed += fuel_needed;
        total_fuel_needed_for_fuel += fuel_requirement_of_fuel(fuel_needed);
    }
    println!("Answer part 1: {}", total_fuel_needed);
    println!("Answer part 2: {}", total_fuel_needed + total_fuel_needed_for_fuel);
}
