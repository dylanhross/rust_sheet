/*
    Module where basic datatypes are defined
*/


#[derive(Debug, Clone)]
pub enum CellVal {
    Int(i32),
    Real(f64),
    Text(String),
    Formula(String),
}


#[derive(Debug, Clone)]
pub struct CellLoc {
    pub col: String,
    pub row: usize,
}


#[derive(Debug)]
pub struct Cell{
    pub val: CellVal,
    pub loc: CellLoc,
}


#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum FormToken {
    Num(f64),
    Loc(CellLoc),
    BinOp(Op),
}


pub type TknLink = Option<Box<TknNode>>;


#[derive(Debug)]
pub struct TknNode {
    pub token: FormToken,
    pub left: TknLink,
    pub right: TknLink,
}

