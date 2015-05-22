extern crate temporary;

use std::{fs, mem};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use self::temporary::Directory;

use threed_ice_sys::*;

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

pub fn setup_simulator<F>(name: Option<&str>, mut code: F)
    where F: FnMut(&mut StackDescription_t, &mut Analysis_t, &mut Output_t) {

    setup_environment(name, move |stack| unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        let mut analysis: Analysis_t = mem::uninitialized();
        let mut output: Output_t = mem::uninitialized();

        stack_description_init(&mut stkd);
        analysis_init(&mut analysis);
        output_init(&mut output);

        success!(parse_stack_description_file(path_to_c_str!(stack).as_ptr() as *mut _,
                                              &mut stkd, &mut analysis, &mut output));

        code(&mut stkd, &mut analysis, &mut output);

        stack_description_destroy(&mut stkd);
        analysis_destroy(&mut analysis);
        output_destroy(&mut output);
    });
}

pub fn setup_environment<F>(name: Option<&str>, mut code: F) where F: FnMut(&Path) {
    let directory = ok!(Directory::new("threed_ice_sys"));
    let from = match name {
        Some(name) => find(name),
        None => find("default"),
    };
    let into = directory.path().join(ok!(from.file_name()));
    copy(&from, &into);
    code(&into);
}

pub fn setup_ping<F>(mut code: F) where F: FnMut() {
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
    use std::ascii::AsciiExt;

    let path = PathBuf::from("tests/fixtures").join(name);
    assert!(ok!(fs::metadata(&path)).is_dir());

    for entry in ok!(fs::read_dir(&path)) {
        let entry = ok!(entry);
        if ok!(fs::metadata(entry.path())).is_dir() {
            continue;
        }
        match &ok!(ok!(entry.path().extension()).to_str()).to_ascii_lowercase()[..] {
            "stk" => return entry.path(),
            _ => {},
        }
    }

    panic!("cannot find a stack description in {:?}", path);
}

fn copy(source: &Path, destination: &Path) {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    let from = ok!(source.parent());
    let into = ok!(destination.parent());

    let mut source = ok!(File::open(source));
    let reader = BufReader::new(&mut source);

    let mut destination = ok!(File::create(destination));
    let mut writer = BufWriter::new(&mut destination);

    for line in reader.lines() {
        let line = ok!(line);
        match line.find('"') {
            Some(i) => match line.rfind('"') {
                Some(j) => {
                    let (prefix, name, suffix) = (&line[..i+1], &line[i+1..j], &line[j..]);
                    let (source, destination) = (from.join(name), into.join(name));

                    ok!(writer.write(prefix.as_bytes()));
                    ok!(writer.write(ok!(destination.to_str()).as_bytes()));
                    ok!(writer.write(suffix.as_bytes()));
                    ok!(writer.write(b"\n"));

                    if fs::metadata(&source).is_ok() {
                        ok!(fs::copy(&source, &destination));
                    }

                    continue;
                },
                _ => assert!(false),
            },
            _ => {},
        }
        ok!(writer.write(line.as_bytes()));
        ok!(writer.write(b"\n"));
    }
}
