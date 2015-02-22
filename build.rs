#![feature(env, path, process)]

use std::{env, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! fmt(
    ($($arg:tt)*) => (format!($($arg)*));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap());
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

fn main() {
    let mut from = PathBuf::new(&get!("CARGO_MANIFEST_DIR"));
    from.push("blas");

    let into = PathBuf::new(&get!("OUT_DIR"));

    run!(cmd!("cmake").current_dir(&into)
                      .arg(&from)
                      .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'"));

    run!(cmd!("cmake").current_dir(&into)
                      .arg("--build").arg(".")
                      .arg("--target").arg("blas")
                      .arg("--")
                      .arg(&fmt!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-flags=-L {}", into.join("lib").display());
    println!("cargo:rustc-flags=-l blas:static");
}
