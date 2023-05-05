use std::{cell::RefCell, fmt::format};

use id_arena::Arena;

use crate::ast::*;

type ValueId = id_arena::Id<Value>;

#[derive(Debug, Clone)]

pub enum IrType {
    Void,
    Int,
}

pub trait ValueTrait {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn ty(&self) -> IrType;
    fn set_ty(&mut self, ty: IrType);
}

macro_rules! impl_value_trait {
    ($type_name:ident) => {
        impl ValueTrait for $type_name {
            fn name(&self) -> String {
                self.name.clone()
            }

            fn set_name(&mut self, name: String) {
                self.name = name;
            }

            fn ty(&self) -> IrType {
                self.ty.clone()
            }

            fn set_ty(&mut self, ty: IrType) {
                self.ty = ty;
            }
        }
    };
}

macro_rules! dummy_value_trait {
    ($type_name:ident) => {
        impl ValueTrait for $type_name {
            fn name(&self) -> String {
                "".to_string()
            }

            fn set_name(&mut self, name: String) {
                
            }

            fn ty(&self) -> IrType {
                IrType::Void
            }

            fn set_ty(&mut self, ty: IrType) {
                
            }
        }
    };
}

macro_rules! impl_value_trait_for_enum {
    ($enum_name:ident { $($variant:ident($variant_ty:ty)),+ $(,)? }) => {
        impl ValueTrait for $enum_name {
            fn name(&self) -> String {
                match self {
                    $( $enum_name::$variant(value) => value.name(), )+
                }
            }

            fn set_name(&mut self, name: String) {
                match self {
                    $( $enum_name::$variant(value) => value.set_name(name), )+
                }
            }

            fn ty(&self) -> IrType {
                match self {
                    $( $enum_name::$variant(value) => value.ty(), )+
                }
            }

            fn set_ty(&mut self, ty: IrType) {
                match self {
                    $( $enum_name::$variant(value) => value.set_ty(ty), )+
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct GlobalValue {
    pub name: String,
    pub ty: IrType,
}

impl_value_trait!(GlobalValue);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantValue {
    Int(i64),
}

impl ValueTrait for ConstantValue {
    fn name(&self) -> String {
        "".to_string()
    }

    fn set_name(&mut self, name: String) {
        
    }

    fn ty(&self) -> IrType {
        match self {
            ConstantValue::Int(_) => IrType::Int,
        }
    }

    fn set_ty(&mut self, ty: IrType) {
        
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, Clone)]

pub struct LoadInst {
    pub name: String,
    pub ty: IrType,
    pub source: ValueId,
}

impl_value_trait!(LoadInst);

#[derive(Debug, Clone)]

pub struct StoreInst {
    pub source: ValueId,
    pub destination: ValueId,
}

dummy_value_trait!(StoreInst);

#[derive(Debug, Clone)]

pub struct AllocaInst {
    pub name: String,
    pub ty: IrType,
}

impl_value_trait!(AllocaInst);


#[derive(Debug, Clone)]
pub struct PrintIntInst {
    pub param: ValueId,
}

dummy_value_trait!(PrintIntInst);

#[derive(Debug, Clone)]

pub struct BinaryOperator {
    pub name: String,
    pub ty: IrType,
    pub operation: BinaryOp,
    pub left_operand: ValueId,
    pub right_operand: ValueId,
}

impl_value_trait!(BinaryOperator);

#[derive(Debug, Clone)]
pub enum Value {
    Global(GlobalValue),
    Instruction(InstructionValue),
    Constant(ConstantValue),
}

impl_value_trait_for_enum!(Value {
    Global(GlobalValue),
    Instruction(InstructionValue),
    Constant(ConstantValue),
});

#[derive(Debug, Clone)]
pub enum InstructionValue {
    BinaryOperator(BinaryOperator),
    LoadInst(LoadInst),
    StoreInst(StoreInst),
    AllocaInst(AllocaInst),
    PrintIntInst(PrintIntInst),
}

impl_value_trait_for_enum!(InstructionValue {
    BinaryOperator(BinaryOperator),
    LoadInst(LoadInst),
    StoreInst(StoreInst),
    AllocaInst(AllocaInst),
    PrintIntInst(PrintIntInst),
});

#[derive(Debug, Clone)]

pub struct Context {
    pub values: RefCell<Arena<Value>>,
    pub instructions: Vec<ValueId>,
    pub next_id: usize,
    pub global_variables: std::collections::HashMap<String, ValueId>,
}

impl Context {
    pub fn new() -> Self {
        let mut context = Self {
            values: RefCell::new(Arena::new()),
            instructions: vec![],
            next_id: 1,
            global_variables: std::collections::HashMap::new(),
        };
        context.create_global_variable("mem".to_string());
        context
    }

    pub fn generate_local_name(&mut self) -> String {
        let name = format!("%{}", self.next_id);
        self.next_id += 1;
        name
    }

    pub fn create_global_variable(&mut self, name: String) -> ValueId {
        let value = Value::Global(GlobalValue {name: format!("@{}", name), ty: IrType::Int});
        let id = self.values.borrow_mut().alloc(value);
        self.global_variables.insert(name, id);
        id
    }
}

pub trait IrGenerator {
    fn to_ir(&self, context: &mut Context);
}

impl IrGenerator for TransUnit {
    fn to_ir(&self, context: &mut Context) {
        self.block.to_ir(context);
    }
}

impl IrGenerator for Block {
    fn to_ir(&self, context: &mut Context) {
        for stmt in &self.stmts {
            stmt.to_ir(context);
        }
    }
}

impl IrGenerator for Stmt {
    fn to_ir(&self, context: &mut Context) {
        match self {
            Stmt::ExprStmt(expr) => {
                let tmp = expr.to_ir(context);
                // save to mem
                let mem = context.global_variables.get("mem").unwrap();
                let store_inst = StoreInst {
                    source: tmp,
                    destination: *mem,
                };
                let inst_value_id = context.values.borrow_mut().alloc(Value::Instruction(
                    InstructionValue::StoreInst(store_inst),
                ));
                context.instructions.push(inst_value_id);
            }
            Stmt::PrintStmt(expr) => {
                let value_id = expr.to_ir(context);
                let print_var_inst = PrintIntInst {
                    param: value_id,
                };
                let inst_value_id = context.values.borrow_mut().alloc(Value::Instruction(
                    InstructionValue::PrintIntInst(print_var_inst),
                ));
                context.instructions.push(inst_value_id);
            }
        }
    }
}

impl Expr {
    fn to_ir(&self, context: &mut Context) -> ValueId {
        match self {
            Expr::Primary(primary_expr) => primary_expr.to_ir(context),
            Expr::Prefix(prefix_expr) => prefix_expr.to_ir(context),
            Expr::Infix(infix_expr) => infix_expr.to_ir(context),
        }
    }
}

impl PrimaryExpr {
    fn to_ir(&self, context: &mut Context) -> ValueId {
        match self {
            PrimaryExpr::Mem => {
                // context.global_variables["mem"]
                // generate a load instruction
                let name =  context.generate_local_name();
                let mem = context.global_variables.get("mem").unwrap();
                let load_inst = LoadInst {
                    name: name,
                    ty: IrType::Int,
                    source: *mem,
                };
                let inst_value_id = context.values.borrow_mut().alloc(Value::Instruction(
                    InstructionValue::LoadInst(load_inst),
                ));
                context.instructions.push(inst_value_id);
                inst_value_id
            }
            PrimaryExpr::Int(i) => {
                let value = Value::Constant(ConstantValue::Int(*i));
                let id = context.values.borrow_mut().alloc(value);
                id
            }
            PrimaryExpr::Expr(expr) => expr.to_ir(context),
        }
    }
}

impl PrefixExpr {
    fn to_ir(&self, context: &mut Context) -> ValueId {
        let expr_value_id = self.expr.to_ir(context);
        match self.op {
            PrefixOp::Plus => expr_value_id,
            PrefixOp::Minus => {
                let zero = context
                    .values
                    .borrow_mut()
                    .alloc(Value::Constant(ConstantValue::Int(0)));
                let bin_op = BinaryOperator {
                    name: context.generate_local_name(),
                    ty: IrType::Int,
                    operation: BinaryOp::Sub,
                    left_operand: zero,
                    right_operand: expr_value_id,
                };
                let id = context
                    .values
                    .borrow_mut()
                    .alloc(Value::Instruction(InstructionValue::BinaryOperator(bin_op)));
                context.instructions.push(id);
                id
            }
        }
    }
}

impl InfixExpr {
    fn to_ir(&self, context: &mut Context) -> ValueId {
        let lhs_value_id = self.lhs.to_ir(context);
        let rhs_value_id = self.rhs.to_ir(context);
        let bin_op = match self.op {
            InfixOp::Plus => BinaryOp::Add,
            InfixOp::Minus => BinaryOp::Sub,
            InfixOp::Multiply => BinaryOp::Mul,
            InfixOp::Divide => BinaryOp::Div,
        };

        let bin_inst = BinaryOperator {
            name: context.generate_local_name(),
            ty: IrType::Int,
            operation: bin_op,
            left_operand: lhs_value_id,
            right_operand: rhs_value_id,
        };
        let id = context.values.borrow_mut().alloc(Value::Instruction(
            InstructionValue::BinaryOperator(bin_inst),
        ));
        context.instructions.push(id);
        id
    }
}
