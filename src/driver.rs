use crate::{parser::parse, interpreter::interpret};

pub fn drive(src: &str) {
    let tu = parse(src);
    if tu.is_err() {
        println!("Error: {:?}", tu.err());
        return;
    }
    interpret(&tu.unwrap());
}