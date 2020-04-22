mod input_finder;
mod program;

use std::env;

use input_finder::InputFinder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let desired_output: u32 = args[2].parse().unwrap();
    
    let input_result = InputFinder::new(filename).find_inputs_for_output(desired_output);

    if let Some(program_input) = input_result {
        println!("Noun {}, Verb {}", program_input.noun, program_input.verb);
        println!("Producing Solution 100 * {} + {} = {}", program_input.noun, program_input.verb, 100 * program_input.noun + program_input.verb);
    } else {
        println!("No inputs found that produce that output!");
    }
}