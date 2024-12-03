#[derive(Debug)]
pub enum BinOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate
}

#[derive(Debug)]
pub enum Expr {
    BinOp {lhs: Box<Expr>, op: BinOperator, rhs: Box<Expr>},
    UnaryOp {arg: Box<Expr>, op: UnaryOperator},
    Integer(i32)
}