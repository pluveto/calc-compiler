use crate::ir::*;

pub trait LlvmEmitter {
    fn emit_ir(&self) -> String;
}
impl LlvmEmitter for Context {
    fn emit_ir(&self) -> String {
        let mut llvm_ir = String::new();

        // Emit print function
        llvm_ir.push_str(r#"
@.str = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1

define dso_local void @print(i64 noundef %0) #0{
  %2 = alloca i64, align 8
  store i64 %0, i64* %2, align 8
  %3 = load i64, i64* %2, align 8
  %4 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([6 x i8], [6 x i8]* @.str, i64 0, i64 0), i64 noundef %3)
  ret void
}

declare i32 @printf(i8*, ...)
"#);

        // Emit global variable declarations
        for (name, id) in &self.global_variables {
            let global_var = format!("@{} = global i64 0\n", name);
            llvm_ir.push_str(&global_var);
        }

        // Emit function definition
        llvm_ir.push_str("define void @main() {\n");

        // Emit instructions
        for inst_id in &self.instructions {
            let value = &self.values.borrow()[*inst_id];
            match value {
                Value::Instruction(instruction) => {
                    let llvm_instruction = emit_instruction(instruction, self);
                    llvm_ir.push_str(&llvm_instruction);
                }
                _ => (),
            }
        }

        // Emit function end
        llvm_ir.push_str("  ret void\n}\n");
        llvm_ir
    }
}


fn emit_instruction(instruction: &InstructionValue, context: &Context) -> String {
    match instruction {
        InstructionValue::LoadInst(load_inst) => {
            let arena = context.values.borrow();
            let src = arena.get(load_inst.source).unwrap();
            format!("  {} = load i64, i64* {}\n", load_inst.name, emit_operand(src, context))
        }
        InstructionValue::StoreInst(store_inst) => {
            let arena = context.values.borrow();
            let dest = arena.get(store_inst.destination).unwrap();
            let src = arena.get(store_inst.source).unwrap();
            format!(
                "  store i64 {}, i64* {}\n",
                emit_operand(src, context),
                emit_operand(dest, context)
            )
        }
        InstructionValue::AllocaInst(alloc_inst) => match alloc_inst.ty {
            IrType::Int => format!("  {} = alloca i64\n", instruction.name()),
            IrType::Void => unreachable!(),
        },
        InstructionValue::BinaryOperator(bin_op) => {
            let operation = match bin_op.operation {
                BinaryOp::Add => "add",
                BinaryOp::Sub => "sub",
                BinaryOp::Mul => "mul",
                BinaryOp::Div => "sdiv",
            };
            let arena = context.values.borrow();
            let left = arena.get(bin_op.left_operand).unwrap();
            let right = arena.get(bin_op.right_operand).unwrap();
            format!(
                "  {} = {} i64 {}, {}\n",
                instruction.name(),
                operation,
                emit_operand(left, context),
                emit_operand(right, context)
            )
        }
        InstructionValue::PrintIntInst(print_var_inst) => {
            let arena = context.values.borrow();
            let param_val = arena.get(print_var_inst.param).unwrap();
            format!(
                "  call void @print(i64 {})\n",
                emit_operand(param_val, context)
            )
        }
    }
}

fn emit_operand(value: &Value, context: &Context) -> String {
    match value {
        Value::Instruction(inst) => inst.name(),
        Value::Global(global) => format!("{}", global.name()),
        Value::Constant(constant) => format!("{}", match constant {
            ConstantValue::Int(int) => int,
        }),
    }
}
