use std::io;
use pest::Parser;
use pest_derive::Parser;
use pest::pratt_parser::PrattParser;
use pest::iterators::Pairs;
mod ast;
use ast::*;
use std::io::BufRead;

#[derive(Parser)]
#[grammar = "simpl.pest"]
pub struct SimPLParser;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    for line in stdin_lock.lines() {
        match SimPLParser::parse(Rule::main, &line?) {
            Ok(mut pairs) => {
                println!(
                    "Parsed: {:#?}",
                    parse_expr(
                        // inner of expr
                        pairs.next().unwrap().into_inner()
                    )
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(negate))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            
            // if primary is an expression (due to parens), recursively parse its inner constituents
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BinOperator::Add,
                Rule::subtract => BinOperator::Subtract,
                Rule::multiply => BinOperator::Multiply,
                Rule::divide => BinOperator::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, arg| {
            let op = match op.as_rule() {
                Rule::negate => UnaryOperator::Negate,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };
            Expr::UnaryOp {arg: Box::new(arg) , op: op }
        })
        .parse(pairs)
}