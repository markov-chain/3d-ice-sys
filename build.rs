#![feature(collections)]

use std::{env, fs, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap_or(String::new()));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

macro_rules! require(
    ($package:expr) => (
        match env::var(&format!("DEP_{}_ROOT", $package.to_uppercase())) {
            Ok(path) => PathBuf::from(path),
            Err(..) => panic!("cannot find `{}`", $package),
        }
    );
);

#[allow(unused_must_use)]
fn main() {
    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("3d-ice");
    let output = PathBuf::from(&get!("OUT_DIR"));
    let superlu = require!("superlu");

    let lib = output.join("lib");
    fs::create_dir(&lib);
    run!(cmd!("make").current_dir(&source)
                     .arg("lib")
                     .arg("CFLAGS=-fPIC -MD -O3 -w")
                     .arg(&format!("SLU_INCLUDE={}", superlu.join("include").display()))
                     .arg("SLU_LIBS=")
                     .arg(&format!("3DICE_LIB_A={}", lib.join("libthreed-ice.a").display())));

    println!("cargo:root={}", output.display());
    println!("cargo:rustc-link-lib=static=threed-ice");
    println!("cargo:rustc-link-search={}", lib.display());
}
