use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        let fuel = calculate_fuel(mass);
        let fuel_fuel = calculate_fuel_fuel(fuel);

        total_fuel += fuel + fuel_fuel;
    }

    println!("{}", total_fuel);
}

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calculate_fuel_fuel(mut fuel: i32) -> i32 {
    let mut fuel_for_fuel = 0;

    while fuel > 0 {
        fuel = calculate_fuel(fuel);
        
        if fuel > 0 {
            fuel_for_fuel += fuel;
        }
    }

    fuel_for_fuel
} 