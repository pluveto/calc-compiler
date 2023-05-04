use crate::ast::*;
struct Env {
    mem: i64,
}

impl Env {
    fn new() -> Self {
        Self {
            mem: 0,
        }
    }

    fn get_mem(&self) -> i64 {
        self.mem
    }

    fn set_mem(&mut self, val: i64) {
        self.mem = val;
    }
}

pub fn interpret(tu: &TransUnit) {
    let mut env = Env::new();
    for stmt in &tu.block.stmts {
        match stmt {
            Stmt::ExprStmt(expr) => {
                let ret = eval_expr(&mut env, expr);
                env.set_mem(ret);
            },
            Stmt::PrintStmt(expr) => {
                println!("{}", eval_expr(&mut env, expr));
            }
        }
    }
  
}

fn eval_expr(env: &mut Env, expr: &Expr) -> i64 {
    match expr {
        Expr::Primary(e)=> eval_primary(env, e),
        Expr::Prefix(e)=> eval_prefix(env, e),
        Expr::Infix(e)=> eval_infix(env, e),
    }
}

fn eval_primary(env: &mut Env, expr: &PrimaryExpr) -> i64 {
    match expr {
        PrimaryExpr::Mem => env.get_mem(),
        PrimaryExpr::Int(i) => *i,
        PrimaryExpr::Expr(e) => eval_expr(env, e),
    }
}

fn eval_prefix(env: &mut Env, expr: &PrefixExpr) -> i64 {
    let rhs = eval_expr(env, &expr.expr);
    match expr.op {
        PrefixOp::Plus => rhs,
        PrefixOp::Minus => -rhs,
    }
}

fn eval_infix(env: &mut Env, expr: &InfixExpr) -> i64 {
    let lhs = eval_expr(env, &expr.lhs);
    let rhs = eval_expr(env, &expr.rhs);
    match expr.op {
        InfixOp::Plus => lhs + rhs,
        InfixOp::Minus => lhs - rhs,
        InfixOp::Multiply => lhs * rhs,
        InfixOp::Divide => lhs / rhs,
    }
}
