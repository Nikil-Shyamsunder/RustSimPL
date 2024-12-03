pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Expr {
    BinOp {op: Op, lhs: Box<Expr>, rhs: Box<Expr>},
    Integer(i32)
}