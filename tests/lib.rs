extern crate temporary;
extern crate threed_ice_sys as raw;

use std::{env, fs};
use std::path::{Path, PathBuf};
use temporary::Directory;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! str_to_c_str(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!(ok!($path.to_str())));
);

mod test_emulator;

fn setup<F>(code: F) where F: Fn(&Path) {
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

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("build/source/bin").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
