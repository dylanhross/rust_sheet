/*
    Module with definition of Sheet class and its methods
*/


use std::io::{self, BufRead, Write};
use std::fs;
use std::path;
use std::mem;
use std::cmp;

use crate::{dtypes, parsing};


fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<path::Path>, {
    // taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // this is supposed to be a lot more efficient than the naive approach which
    // involves putting every line of the file into Strings in memory 
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[derive(Debug)]
pub struct Sheet {
    cols: Vec<Vec<dtypes::Cell>>,
    pub n_cols: usize,
    pub n_rows: usize,
}


impl Sheet {
    pub fn new () -> Sheet {
        let cols: Vec<Vec<dtypes::Cell>> = Vec::new();
        let sheet = Sheet {
            cols,
            // the data in the sheet are stored sparse so only
            // cells with actual values are explicitly stored
            // n_cols and n_rows reflect the number of columns
            // and rows that would be needed to fully contain
            // all of the cells with explicit values
            n_cols: 0,
            n_rows: 0,
        };
        sheet
    }

    pub fn load_sheet (&mut self) {
        eprintln!("loading sheet state from file (sheet.txt)");
        // File sheet.txt must exist in the current path
        if let Ok(lines) = read_lines("./sheet.txt") {
            let mut n_cells: usize = 0;
            // Consumes the iterator, returns an (Optional) String
            for (i, line) in lines.enumerate() {
                if i == 0 {
                    // parse the first line as "<n_cols> <n_rows>"
                    if let Ok(line) = line {
                        let (n_cols, n_rows) = parsing::parse_first_line(&line);
                        self.n_rows = n_rows;
                        // add enough column vectors to match the specified sheet dimensions
                        while self.cols.len() < n_cols {
                            self.add_col();
                        }
                        eprintln!("loaded dimensions: {} cols, {} rows", n_cols, n_rows);
                    }
                } else {
                    // parse all of the rest of the lines as "<loc> <val>"
                    
                    if let Ok(line) = line {
                        let (loc, val) = parsing::parse_line(&line);
                        self.write_cell(loc, val);
                        n_cells += 1;
                    }
                }
            }
            eprintln!("loaded {} cells", n_cells);
        }
    }

    pub fn save_sheet (&self) {
        eprintln!("saving sheet");
        let file = fs::File::create("./sheet.txt").unwrap();
        let mut buf = io::BufWriter::new(file);
        // first print <n_cols> <n_rows>s
        let _ = buf.write_fmt(format_args!("{} {}\n", self.n_cols, self.n_rows)).unwrap();
        for col in &self.cols {
            for cell in col {
                let _ = buf.write_fmt(format_args!("{}{} {:?}\n", cell.loc.col, cell.loc.row, cell.val)).unwrap();
            }
        }
        let _ = buf.flush().unwrap();
    }

    fn col_to_index (col: &String) -> usize {
        let mut idx: usize = 0;
        let base: usize = 26;
        // iterate index letters from right to left 
        for (i, c) in col.chars().rev().enumerate() {
            idx += base.pow(i as u32) * (c as usize - 64);
        }
        // 0 indexed
        idx - 1
    }

    pub fn add_col (&mut self) {
        self.cols.push(Vec::new());
        self.n_cols += 1;
    }

    pub fn add_row (&mut self) {
        self.n_rows += 1;
    }

    pub fn write_cell (&mut self, loc: dtypes::CellLoc, val: dtypes::CellVal) {
        // find out the column index, add columns if it is 
        // beyond the current bounds of the sheet
        let col_idx = Sheet::col_to_index(&loc.col);
        while col_idx >= self.n_cols {
            self.add_col();
        }
        // now there are for sure enough columns in the sheet
        // add the cell to the appropriate position in the column
        // vector (sorted by row number)
        let col = &mut self.cols[col_idx];
        let row = loc.row;
        let new_cell = dtypes::Cell {loc, val};
        if col.len() == 0 || row > col.last().unwrap().loc.row {
            // add to end if cols is empty or row greater than row of last cell
            col.push(new_cell);
        } else {
            // determine the insertion index of the cell
            let mut ins_idx: usize = 0;
            let mut ins_flg = true;
            for cell in col.iter() {
                if cell.loc.row == row {
                    ins_flg = false;
                    break;
                }
                if row < cell.loc.row {
                    break;
                } else {
                    ins_idx += 1;
                }
            }
            //
            if ins_flg {
                col.insert(ins_idx, new_cell);
            } else {
                // replace returns the element that was replaced
                // so store that in a throwaway variable
                let _ = mem::replace(&mut col[ins_idx], new_cell);
            }
        }
        // update Sheet.n_rows if needed
        if row > self.n_rows {
            self.n_rows = row;
        }
    }

    fn eval_tree (&self, root: dtypes::TknLink) -> Option<f64> {
        match root {
            Some(node) => {
                match node.token {
                    dtypes::FormToken::Num(num) => Option::Some(num),
                    dtypes::FormToken::Loc(loc) => {
                        if let Some(cv) = self.get_cell(loc) {
                            match cv {
                                dtypes::CellVal::Int(v) => {
                                    Option::Some(v as f64)
                                },    
                                dtypes::CellVal::Real(v) => {
                                    Option::Some(v)
                                },
                                _ => Option::None
                            }
                        } else {
                            Option::None
                        }
                    },
                    dtypes::FormToken::BinOp(op) => {
                        match op {
                            dtypes::Op::Plus => {
                                if let Some(left_val) = self.eval_tree(node.left) {
                                    if let Some(right_val) = self.eval_tree(node.right) {
                                        Option::Some(left_val + right_val) 
                                    } else {
                                        Option::None
                                    }
                                } else {
                                    Option::None
                                }
                            },
                            dtypes::Op::Minus => {
                                if let Some(left_val) = self.eval_tree(node.left) {
                                    if let Some(right_val) = self.eval_tree(node.right) {
                                        Option::Some(left_val - right_val) 
                                    } else {
                                        Option::None
                                    }
                                } else {
                                    Option::None
                                }
                            },
                        }
                    },
                }
            },
            // empty tree -> return no value
            None => Option::None,
        }  
    }

    pub fn eval_formula_cell (&self, cell_val: &dtypes::CellVal) -> dtypes::CellVal {
        // step 1: parse into token tree
        let tree_res = parsing::parse_formula_expr(cell_val);
        match tree_res {
            Some(tree) => {
                // step 2: evaluate token tree into a cell value
                let cv_res = self.eval_tree(tree.root);
                match cv_res {
                    Some(cv) => dtypes::CellVal::Real(cv),
                    None => dtypes::CellVal::Text(String::from("#ERR")),
                }
            },
            None => dtypes::CellVal::Text(String::from("#ERR")),
        }
    }
    
    pub fn read_sheet (&self) {
        // first print <n_cols> <n_rows>
        println!("{} {}", self.n_cols, self.n_rows);
        // then print all cell values
        // formulas are evaluated at this point and evaluated values
        // are printed
        for col in &self.cols {
            for cell in col {
                // print value of Int, Real, and Text cells to stdout
                // evaluate and print result of Formula cells
                match cell.val {
                    dtypes::CellVal::Formula(_) => {
                        let display_val = self.eval_formula_cell(&cell.val);
                        println!("{}{} {:?}", cell.loc.col, cell.loc.row, display_val);
                    }
                    _ => {
                        println!("{}{} {:?}", cell.loc.col, cell.loc.row, cell.val);
                    }
                }
            }
        }
    }

    fn get_cell (&self, loc: dtypes::CellLoc) -> Option<dtypes::CellVal> {
        let col_idx = Sheet::col_to_index(&loc.col);
        if col_idx < self.n_cols {
            let col = &self.cols[col_idx];
            for cell in col {
                if cell.loc.row == loc.row {
                    eprintln!("found a cell at loc: {:?}", loc);
                    return Option::Some(cell.val.clone())
                }
            }
        }
        eprintln!("did not find a cell at loc: {:?}", loc);
        Option::None
    }

    pub fn read_cell (&self, loc: dtypes::CellLoc) {
        // prints the value of the selected cell to stdout
        // prints nothing if there is no cell there
        // Importantly, for formulas this is the the raw value (i.e. the formula
        // definition) instead of the evaluated value that is printed by read_sheet
        if let Some(cell_val) = self.get_cell(loc) {
            println!("{:?}", cell_val);
        }
    }

    pub fn delete_cell (&mut self, loc: dtypes::CellLoc) -> bool {
        // deletes the selected cell
        // do nothing if there is no cell there
        // returns a bool indicating whether a cell was deleted or not
        let mut found_cell = false;
        let mut rm_idx: usize = 0;
        let col_idx = Sheet::col_to_index(&loc.col);
        if col_idx < self.n_cols {
            let col = &self.cols[col_idx];
            for (i, cell) in col.iter().enumerate() {
                if cell.loc.row == loc.row {
                    eprintln!("found a cell at loc: {:?}", loc);
                    found_cell = true;
                    rm_idx = i;
                    break;
                }
            }
        }
        // report to stderr that we could not find a cell
        if found_cell {
            // remove returns the removed element 
            // so just store in a throwaway variable
            let _ = self.cols[col_idx].remove(rm_idx);
            eprintln!("deleted cell")
        } else {
            eprintln!("did not find a cell at loc: {:?}", loc);
            eprintln!("nothing to delete")
        }
        found_cell
    }

    pub fn clear_sheet (&mut self) {
        // completely clears out the sheet by rewriting sheet.txt 
        // then reloading it
        eprintln!("clearing sheet");
        let file = fs::File::create("./sheet.txt").unwrap();
        let mut buf = io::BufWriter::new(file);
        // first print <n_cols> <n_rows>s
        let _ = buf.write(b"0 0\n").unwrap();
        let _ = buf.flush().unwrap();
    }

    pub fn shrink (&mut self) -> bool {
        // figure out how many columns at the end of cols 
        // are empty and can be removed
        // returns a bool indicating if any changes were made
        let mut modified = false;
        let mut trim_cols: usize = 0;
        for col in self.cols.iter().rev() {
            if col.is_empty() {
                trim_cols += 1;
            }
        }
        eprintln!("removing {} empty columns from the end", trim_cols);
        while trim_cols > 0 {
            let _ = self.cols.pop();
            trim_cols -= 1;
            self.n_cols -= 1;
            modified = true;
        }
        // set self.n_rows to whatever the maximum row is 
        let mut max_row: usize = 0;
        for col in self.cols.iter() {
            for cell in col {
                max_row = cmp::max(cell.loc.row, max_row);
            }
        }
        if self.n_rows > max_row {
            modified = true;
        }
        eprintln!("shrinking rows from {} to {}", self.n_rows, max_row);
        self.n_rows = max_row;
        modified
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_eval_tree () {
        // init the sheet
        let mut sheet = Sheet::new();
        // load sheet state from file
        sheet.load_sheet();
        // build a tree from a formula
        let cell_val = dtypes::CellVal::Formula(String::from("=1-A1+2-B3+3"));
        let tree = parsing::parse_formula_expr(&cell_val).unwrap();
        eprintln!("--------------------");
        eprintln!("tree: {:?}", tree);
        eprintln!("--------------------");
        if let Some(eval_cell_val) = sheet.eval_tree(tree.root) {
            println!("eval_cell_val: {:?}", eval_cell_val);
        }
        eprintln!("--------------------");
        // build a tree from a formula
        let cell_val = dtypes::CellVal::Formula(String::from("=1+2+3-4+5"));
        let tree = parsing::parse_formula_expr(&cell_val).unwrap();
        eprintln!("--------------------");
        eprintln!("tree: {:?}", tree);
        eprintln!("--------------------");
        if let Some(eval_cell_val) = sheet.eval_tree(tree.root) {
            println!("eval_cell_val: {:?}", eval_cell_val);
        }
        eprintln!("--------------------");
    }
}
