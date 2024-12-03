use pest::Parser;
use pest_derive::Parser;
mod ast;

#[derive(Parser)]
#[grammar = "simpl.pest"]
pub struct SimPLParser;

fn main() {
    println!("Hello, world!");
}
