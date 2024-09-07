#![allow(dead_code)]
use poke_engine::io;
use std::process::exit;

extern crate lazy_static;

fn main() {
    io::main();
    exit(1);
}
