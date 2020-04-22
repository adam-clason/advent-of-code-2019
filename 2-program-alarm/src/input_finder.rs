use crate::program::{Program, ProgramInput};

pub struct InputFinder<'a> {
    input_file_name: &'a str,
}

impl<'a> InputFinder<'a> {

    pub fn new(file_name: &'a str) -> Self {
        InputFinder {
            input_file_name: file_name
        }
    }


    pub fn find_inputs_for_output(&self, desired_output: u32) -> Option<ProgramInput> {
        for n in 1..99 {
            for v in 1..99 {
                let program_input = ProgramInput {
                    noun: n,
                    verb: v,
                };

                if let Ok(program) = Program::new_from_file(self.input_file_name) {
                    let output = program.with_input(&program_input).run();
                    
                    if output == desired_output {
                        return Some(program_input)
                    }
                }
            }
        }

        None
    }

}