pub struct TransUnit {
    pub block: Block,
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
}
pub enum Expr {
    Primary(Box<PrimaryExpr>),
    Prefix(Box<PrefixExpr>),
    Infix(Box<InfixExpr>),
}

pub struct PrefixExpr {
    pub op: PrefixOp,
    pub expr: Box<Expr>,
}

pub struct InfixExpr {
    pub lhs: Box<Expr>,
    pub op: InfixOp,
    pub rhs: Box<Expr>,
}

pub enum PrefixOp {
    Plus,
    Minus,
}

pub enum InfixOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub enum PrimaryExpr {
    Mem,
    Int(i64),
    Expr(Box<Expr>),
}
