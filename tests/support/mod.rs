use ffi::*;
use fixture;
use std::mem;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use temporary::Directory;

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

pub fn deploy(name: &str) -> (PathBuf, Directory) {
    let source = find(name);
    let directory = ok!(Directory::new("threed_ice_sys"));
    let destination = directory.path().join(ok!(source.file_name()));
    ok!(fixture::copy::with_references(&source, &destination));
    (destination, directory)
}

pub fn setup<F>(name: Option<&str>, mut code: F)
    where F: FnMut(&mut StackDescription_t, &mut Analysis_t, &mut Output_t)
{
    let (path, _directory) = deploy(name.unwrap_or("default"));

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

pub fn ping<F>(mut code: F) where F: FnMut() {
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
    let path = PathBuf::from("tests/fixtures").join(name);
    match fixture::find::first_with_extension(&path, "stk") {
        Some(path) => path,
        None => panic!("cannot find a stack description in {:?}", path),
    }
}
