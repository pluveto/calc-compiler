use crate::{parser::parse, ir::*, codegen::LlvmEmitter};

pub fn drive(src: &str) {
    let tu = parse(src);
    if tu.is_err() {
        println!("Error: {:?}", tu.err());
        return;
    }
    // interpret(&tu.unwrap());
    let tu = tu.unwrap();
    let mut ir_ctx = crate::ir::Context::new();
    tu.to_ir(&mut ir_ctx);
    println!("{}", ir_ctx.emit_ir());
}
