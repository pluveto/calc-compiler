use crate::ast::*;
use pest::iterators::Pair;
use pest::{pratt_parser::PrattParser, Parser};

#[derive(Parser, Default)]
#[grammar = "calc.pest"]
pub struct CalcParser {}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        // Precedence is defined lowest to highest
        PrattParser::new()
        .op(Op::prefix(Rule::prefix_op))
        .op(Op::infix(Rule::infix_op, Left))
        .op(Op::postfix(Rule::postfix_op))
    };
}

// grammar = { trans_unit ~ EOI }
pub fn parse(src: &str) -> Result<TransUnit, pest::error::Error<Rule>> {
    let mut grammar_pairs = CalcParser::parse(Rule::grammar, src)?;
    let tu = parse_grammar(grammar_pairs.next().unwrap())?;
    Ok(tu)
}

pub fn parse_grammar(pair: Pair<Rule>) -> Result<TransUnit, pest::error::Error<Rule>> {
    let tu = pair.into_inner().next().unwrap();
    parse_trans_unit(tu)
}

// trans_unit = { block }
pub fn parse_trans_unit(pair: Pair<Rule>) -> Result<TransUnit, pest::error::Error<Rule>> {
    let block = pair.into_inner().next().unwrap();
    Ok(TransUnit {
        block: parse_block(block)?,
    })
}

// block = { "{" ~ stmts ~ "}" }
fn parse_block(pair: Pair<Rule>) -> Result<Block, pest::error::Error<Rule>> {
    let inner = pair.into_inner();

    let mut statements = Vec::new();
    for p in inner {
        statements.push(parse_statement(p)?);
    }
    Ok(Block { stmts: statements })
}

// stmt = { expr_stmt | print_stmt }
fn parse_statement(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::expr_stmt => parse_expr_statement(inner),
        Rule::print_stmt => parse_print_statement(inner),
        _ => unreachable!(),
    }
}

// expr_stmt = { expr ~ ";" }
fn parse_expr_statement(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>> {
    let inner = pair.into_inner().next().unwrap();
    let expr = parse_expr(inner)?;
    Ok(Stmt::ExprStmt(expr))
}

// print_stmt = { "print" ~ expr ~ ";" }
fn parse_print_statement(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>> {
    let inner = pair.into_inner().next().unwrap();
    let expr = parse_expr(inner)?;
    Ok(Stmt::PrintStmt(expr))
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, pest::error::Error<Rule>> {
    let inner = pair.into_inner();
    let expr = PRATT_PARSER
        .map_primary(|x| Expr::Primary(Box::new(parse_primary_expr(x).unwrap())))
        .map_infix(|lhs, op, rhs| {
            Expr::Infix(Box::new(InfixExpr {
                lhs: Box::new(lhs),
                op: parse_infix(op).unwrap(),
                rhs: Box::new(rhs),
            }))
        })
        .map_prefix(|op, rhs| {
            Expr::Prefix(Box::new(PrefixExpr {
                op: parse_prefix(op).unwrap(),
                expr: Box::new(rhs),
            }))
        })
        .parse(inner.into_iter());
    Ok(expr)
}

// prefix_op = { "+" | "-" }
fn parse_prefix(op: Pair<Rule>) -> Result<PrefixOp, pest::error::Error<Rule>> {
    match op.as_str() {
        "+" => Ok(PrefixOp::Plus),
        "-" => Ok(PrefixOp::Minus),
        _ => unreachable!(),
    }
}

// infix_op = { "+" | "-" | "*" | "/" }
fn parse_infix(op: Pair<Rule>) -> Result<InfixOp, pest::error::Error<Rule>> {
    match op.as_str() {
        "+" => Ok(InfixOp::Plus),
        "-" => Ok(InfixOp::Minus),
        "*" => Ok(InfixOp::Multiply),
        "/" => Ok(InfixOp::Divide),
        _ => unreachable!(),
    }
}

// primary_expr = { INT | "(" ~ expr ~ ")" }
fn parse_primary_expr(pair: Pair<Rule>) -> Result<PrimaryExpr, pest::error::Error<Rule>> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::MEM => Ok(PrimaryExpr::Mem),
        Rule::INT => parse_int(inner),
        Rule::expr => Ok(PrimaryExpr::Expr(Box::new(parse_expr(inner)?))),
        _ => unreachable!(),
    }
}

// INT = { ASCII_DIGIT+ }
fn parse_int(pair: Pair<Rule>) -> Result<PrimaryExpr, pest::error::Error<Rule>> {
    let int = pair.as_str().parse::<i64>().unwrap();
    Ok(PrimaryExpr::Int(int))
}
