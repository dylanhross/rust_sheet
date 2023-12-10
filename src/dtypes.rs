/*
    Module where basic datatypes are defined
*/


#[derive(Debug)]
pub enum CellVal {
    Int(i32),
    Real(f64),
    Text(String),
    Formula(String),
}


#[derive(Debug)]
pub struct CellLoc {
    pub col: String,
    pub row: usize,
}


#[derive(Debug)]
pub struct Cell{
    pub val: CellVal,
    pub loc: CellLoc,
}


#[derive(Debug)]
pub enum FormToken {
    Num(f64),
    Loc(CellLoc),
    Op(String),
}


#[derive(Debug)]
pub struct FormNode {
    pub token: FormToken,

}
