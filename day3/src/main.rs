use std::io::{self, BufRead};
use std::collections::HashMap;

fn locations_from_path(wire_path: String) -> Vec<(i32, i32)> {
    /* Create a vector of all locations reached by a wire path
       in the order they are reached.

       Would have liked to use a generator for this, but they
       are currently only in nightly builds.
    */
    let mut curr_location = (0, 0);
    let mut locations: Vec<(i32, i32)> = Vec::new();
    for action in wire_path.split(',') {
        let direction = action.chars().next().unwrap();
        let steps: i32 = action[1..].parse().unwrap();
        for _ in 0..steps{
            match direction {
                'R' => curr_location.0+=1,
                'L' => curr_location.0-=1,
                'U' => curr_location.1+=1,
                'D' => curr_location.1-=1,
                _ => panic!("Unexpected direction {}", direction)
            };
            locations.push(curr_location.clone());
        }
    }
    locations
}

fn main() {

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let wire1 = lines.next().unwrap().unwrap();
    let wire2 = lines.next().unwrap().unwrap();

    // Create a HashMap from locations wire 1 has reached to the number
    // of steps the signal has traveled to get there.
    let mut wire1_locations: HashMap<(i32,i32), usize> = HashMap::new();
    for (step, location) in (1..).zip(locations_from_path(wire1).iter()) {
        if !wire1_locations.contains_key(&location){
            wire1_locations.insert(location.clone(), step);
        }
    }

    // Create two vectors of intersection distances:
    // One for manhattan distances and one for the combined signal steps
    // of wire 1 and 2 at the intersection.
    let mut manhattan_distances: Vec<i32> = Vec::new();
    let mut signal_steps: Vec<usize> = Vec::new();
    for (step, location) in (1..).zip(locations_from_path(wire2).iter()) {
        let wire1_steps = wire1_locations.get(&location);
        match wire1_steps {
            Some(x) => {
                manhattan_distances.push(location.0.abs() + location.1.abs());
                signal_steps.push(step + x);
            },
            _ => continue
        }
    }

    println!("Answer part 1: {}", manhattan_distances.iter().min().unwrap());
    println!("Answer part 2: {}", signal_steps.iter().min().unwrap());
}
