#![allow(dead_code)]
use std::process::exit;
use poke_engine::io;

extern crate lazy_static;

fn main() {
    io::main();
    exit(1);
}
