mod ir;
mod ast;
mod parser;
mod driver;
mod interpreter;
mod codegen;

#[macro_use]
extern crate pest_derive;
use std::env;

use driver::drive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => print!("Usage: calc -e EXPR\n"),
            "-v" | "--version" => print!("calc 0.1.0\n"),
            "-e" | "--expr" => {
                i += 1;
                drive(args[i].as_str());
            }
            _ => {
                print!("calc: Unrecognized option '{}'\n", args[i])
            }
        }
        i += 1;
    }
}
