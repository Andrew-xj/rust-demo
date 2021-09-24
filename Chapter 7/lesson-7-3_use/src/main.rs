use std::collections::HashMap;

fn main() {
    println!("See lib.rs!");

    let mut map = HashMap::new();
    map.insert(1, 2);
}

/*
// use parent mod
use std::fmt;
use std::io;
fn f1() -> fmt::Result {}
fn f2() -> io::Result {}

// use "as" key word
use std::fmt::Result;
use std::io::Result as IoResult;
fn f1() -> Result {}
fn f2() -> IoResult {}
*/
