#![feature(path_ext)]

extern crate temporary;
extern crate threed_ice_sys as raw;

use std::{env, fs, mem};
use std::ffi::CString;
use std::path::PathBuf;
use temporary::Directory;

use raw::*;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! str_to_c_str(
    ($str:expr) => (CString::new($str).unwrap());
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!($path.to_str().unwrap()));
);

// https://github.com/copies/3d-ice/blob/master/bin/3D-ICE-Emulator.c
#[test]
fn emulator() {
    let (stack, directory) = setup();
    ok!(env::set_current_dir(directory.path()));

    unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        let mut analysis: Analysis_t = mem::uninitialized();
        let mut output: Output_t = mem::uninitialized();

        stack_description_init(&mut stkd);
        analysis_init(&mut analysis);
        output_init(&mut output);

        let error = parse_stack_description_file(
            path_to_c_str!(stack).as_ptr() as *mut _,
            &mut stkd, &mut analysis, &mut output,
        );
        assert!(error == TDICE_SUCCESS);

        stack_description_destroy(&mut stkd);
        output_destroy(&mut output);
    }
}

fn setup() -> (PathBuf, Directory) {
    const CORE: &'static str = "core.flp";
    const MEMORY: &'static str = "mem.flp";
    const STACK: &'static str = "example_transient.stk";

    let directory = ok!(Directory::new("3d-ice"));

    ok!(fs::copy(find(CORE), directory.path().join(CORE)));
    ok!(fs::copy(find(MEMORY), directory.path().join(MEMORY)));
    ok!(fs::copy(find(STACK), directory.path().join(STACK)));

    (directory.path().join(STACK), directory)
}

fn find(name: &str) -> PathBuf {
    use std::fs::PathExt;
    let path = PathBuf::from("3d-ice/bin").join(name);
    assert!(path.exists());
    path
}
