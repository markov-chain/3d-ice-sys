extern crate temporary;
extern crate threed_ice_sys as raw;

use std::{env, fs, mem};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use temporary::Directory;

use raw::*;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! success(
    ($result:expr) => (assert!($result == TDICE_SUCCESS));
);

macro_rules! str_to_c_str(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!(ok!($path.to_str())));
);

mod test_emulator;
mod test_system_matrix;

fn setup_simulator<F>(mut code: F)
    where F: FnMut(&mut StackDescription_t, &mut Analysis_t, &mut Output_t) {

    setup_environment(move |stack| unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        let mut analysis: Analysis_t = mem::uninitialized();
        let mut output: Output_t = mem::uninitialized();

        stack_description_init(&mut stkd);
        analysis_init(&mut analysis);
        output_init(&mut output);

        success!(parse_stack_description_file(path_to_c_str!(stack).as_ptr() as *mut _,
                                              &mut stkd, &mut analysis, &mut output));

        assert!(analysis.AnalysisType == TDICE_ANALYSIS_TYPE_TRANSIENT);

        code(&mut stkd, &mut analysis, &mut output);

        stack_description_destroy(&mut stkd);
        analysis_destroy(&mut analysis);
        output_destroy(&mut output);
    });
}

fn setup_environment<F>(mut code: F) where F: FnMut(&Path) {
    const CORE: &'static str = "core.flp";
    const MEMORY: &'static str = "mem.flp";
    const STACK: &'static str = "example_transient.stk";

    let directory = ok!(Directory::new("3d-ice-sys"));

    ok!(fs::copy(find(CORE), directory.path().join(CORE)));
    ok!(fs::copy(find(MEMORY), directory.path().join(MEMORY)));
    ok!(fs::copy(find(STACK), directory.path().join(STACK)));

    let current_path = ok!(env::current_dir());

    ok!(env::set_current_dir(directory.path()));
    code(&directory.path().join(STACK));
    ok!(env::set_current_dir(&current_path));
}

fn setup_ping<F>(mut code: F) where F: FnMut() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        loop {
            println!("Working...");
            match rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => {},
                _ => break,
            }
            thread::sleep_ms(10 * 1000);
        }
    });

    code();

    ok!(tx.send(true));
    ok!(handle.join());
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("build/source/bin").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
