mod path; 
mod movement;
mod line;
mod coordinate;

use std::fs;
use std::env;

use path::Path;
use coordinate::Coordinate;

fn read_movement_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let mut movement_data: Vec<Vec<String>> = 
        fs::read_to_string(filename)
            .unwrap()
            .trim()
            .lines()
            .map(|s| s.split(',').map(|s| s.to_owned()).collect())
            .collect();

    (movement_data.remove(0), movement_data.remove(0))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let (movement_input_1, movement_input_2) = read_movement_input(filename);   
    
    let path_1 = Path::generate(movement_input_1);
    let path_2 = Path::generate(movement_input_2);
    
    if let Some(intersect) = path_1.closest_intersect(&path_2) {
        println!("Shortest Distance: {:?}", intersect.distance_to(&Coordinate::origin()));
    }

    if let Some(total_distance) = path_1.shortest_walk(&path_2) {
        println!("Shortest Walk: {}", total_distance);
    }
}
