extern crate fixture;
extern crate temporary;

use ffi::*;
use std::path::PathBuf;
use std::{env, mem};

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! success(
    ($result:expr) => (assert!($result == TDICE_SUCCESS));
);

macro_rules! str_to_cstr(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_cstr(
    ($path:expr) => (str_to_cstr!(ok!($path.to_str())));
);

pub fn deploy(name: &str) -> (PathBuf, temporary::Directory) {
    let source = find(name);
    let directory = ok!(temporary::Directory::new("threed_ice_sys"));
    let destination = directory.path().join(ok!(source.file_name()));
    ok!(fixture::copy::with_references(&source, &destination));
    (destination, directory)
}

pub fn setup<F>(mut code: F)
    where F: FnMut(&mut StackDescription_t, &mut Analysis_t, &mut Output_t)
{
    let (path, directory) = deploy("default");
    env::set_current_dir(&directory).unwrap();

    unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        let mut analysis: Analysis_t = mem::uninitialized();
        let mut output: Output_t = mem::uninitialized();

        stack_description_init(&mut stkd);
        analysis_init(&mut analysis);
        output_init(&mut output);

        success!(parse_stack_description_file(path_to_cstr!(path).as_ptr() as *mut _,
                                              &mut stkd, &mut analysis, &mut output));

        code(&mut stkd, &mut analysis, &mut output);

        stack_description_destroy(&mut stkd);
        analysis_destroy(&mut analysis);
        output_destroy(&mut output);
    }
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/fixtures").join(name);
    match fixture::find::first_with_extension(&path, "stk") {
        Some(path) => path,
        None => panic!("cannot find a stack description in {:?}", path),
    }
}
