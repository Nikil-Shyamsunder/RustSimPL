#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Expr {
    BinOp {lhs: Box<Expr>, op: Op, rhs: Box<Expr>},
    Integer(i32)
}