use ffi::*;
use std::mem;

#[test]
fn test_invalid() {
    let (path, _directory) = ::support::deploy("invalid");

    unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        let mut analysis: Analysis_t = mem::uninitialized();
        let mut output: Output_t = mem::uninitialized();

        stack_description_init(&mut stkd);
        analysis_init(&mut analysis);
        output_init(&mut output);

        let result = parse_stack_description_file(path_to_cstr!(path).as_ptr() as *mut _,
                                                  &mut stkd, &mut analysis, &mut output);

        assert!(result != TDICE_SUCCESS);

        stack_description_destroy(&mut stkd);
        analysis_destroy(&mut analysis);
        output_destroy(&mut output);
    }
}
