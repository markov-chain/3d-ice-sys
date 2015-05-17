use analysis::*;
use output::*;
use stack_description::*;
use types::*;

extern "C" {
    pub fn parse_stack_description_file(filename: String_t,
                                        stkd: *mut StackDescription_t,
                                        analysis: *mut Analysis_t,
                                        output: *mut Output_t) -> Error_t;
}
