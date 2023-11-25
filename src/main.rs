/*  */


use std::env;
use std::process;
use std::mem;


#[derive(Debug)]
enum CellVal {
    Int(i32),
    Real(f64),
    Text(String),
}

#[derive(Debug)]
struct CellLoc {
    col: String,
    row: usize,
}


#[derive(Debug)]
struct Cell{
    val: CellVal,
    loc: CellLoc,
}


#[derive(Debug)]
struct Sheet {
    cols: Vec<Vec<Cell>>,
    n_cols: usize,
    n_rows: usize,
}

impl Sheet {
    fn new () -> Sheet {
        let cols: Vec<Vec<Cell>> = Vec::new();
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

    fn add_col (&mut self) {
        self.cols.push(Vec::new());
        self.n_cols += 1;
    }

    fn add_row (&mut self) {
        self.n_rows += 1;
    }

    fn write_cell (&mut self, loc: CellLoc, val: CellVal) {
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
        let new_cell = Cell {loc, val};
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

    fn read_cell (&mut self, loc: CellLoc) {
        // prints the value of the selected cell to stdout
        // prints nothing if there is no cell there
        let mut found_cell = false;
        let col_idx = Sheet::col_to_index(&loc.col);
        if col_idx < self.n_cols {
            let col = &self.cols[col_idx];
            for cell in col {
                if cell.loc.row == loc.row {
                    eprintln!("found a cell at loc: {:?}", loc);
                    println!("{:?}", cell.val);
                    found_cell = true;
                    break;
                }
            }
        }
        // report to stderr that we could not find a cell
        if !found_cell {
            eprintln!("did not find a cell at loc: {:?}", loc);
        }
    }
}


fn parse_loc (loc_arg: &String) -> CellLoc {
    let mut buf_col = String::new();
    let mut buf_row = String::new();
    let mut number_flag = false;
    let mut letter_flag = false;
    for c in loc_arg.chars() {
        if c.is_alphabetic() {
            letter_flag = true;
            buf_col.push(c.to_ascii_uppercase());  // convert to uppercase letter if not already
            // simple check to make sure the location is in a usable form
            // just make sure that is is composed of letters then numbers
            if number_flag {
                eprintln!("bad cell location: {}", loc_arg);
                process::exit(1);
            }
        }
        else if c.is_numeric() {
            // another simple check to make sure there were actually letters first
            if !letter_flag {
                eprintln!("bad cell location: {}", loc_arg);
                process::exit(1);
            }
            number_flag = true;
            buf_row.push(c);
        }
    }
    // another simple check to make sure there were numbers
    if !number_flag {
        eprintln!("bad cell location: {}", loc_arg);
        process::exit(1);
    }
    CellLoc {
        col: buf_col,
        row: buf_row.parse::<usize>().unwrap(),
    }
}


fn parse_val (val_arg: &String) -> CellVal {
    match val_arg.parse::<i32>() {  // try parse as int first
        Ok(val) => CellVal::Int(val),
        _ => match val_arg.parse::<f64>() {  // try parse as real next
            Ok(val) => CellVal::Real(val),
            _ => CellVal::Text(val_arg.clone())  // otherwise parse as text
        }
    }
}


fn handle_subcommand (subcommand: &String, other_args: &[String], sheet: &mut Sheet) {
    let n_other_args = other_args.len();
    match subcommand.as_str() {
        "write_cell" => {
            if n_other_args != 2 {
                eprintln!("write_cell subcommand takes 2 args: <loc> <value>");
                process::exit(1);
            }
            eprintln!("subcommand: {}", subcommand);
            let loc = parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
            let val = parse_val(&other_args[1]);
            eprintln!("parsed cell value: {:?}", val);
            sheet.write_cell(loc, val);
        },
        "read_cell" => {
            if n_other_args != 1 {
                eprintln!("read_cell subcommand takes 1 arg: <loc>");
                process::exit(1);
            }
            eprintln!("subcommand: {}", subcommand);
            let loc = parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
            sheet.read_cell(loc);
        },
        "count_rows" => {
            eprintln!("subcommand: {}", subcommand);
            // print the number of rows in the sheet to stdout
            println!("{}", sheet.n_rows);
        },
        "count_cols" => {
            eprintln!("subcommand: {}", subcommand);
            // print the number of cols in the sheet to stdout
            println!("{}", sheet.n_cols);
        }
        "add_row" => {
            eprintln!("subcommand: {}", subcommand);
            eprintln!("rows before: {}", sheet.n_rows);
            sheet.add_row();
            eprintln!("rows after: {}", sheet.n_rows);
        },
        "add_col" => {
            eprintln!("subcommand: {}", subcommand);
            // print the number of cols in the sheet to stdout
            eprintln!("cols before: {}", sheet.n_cols);
            sheet.add_col();
            eprintln!("cols after: {}", sheet.n_cols);
        }
        _ => {
            eprintln!("unrecognized subcommand: {}", subcommand);
            process::exit(1);
        },
    }
}

fn main () {

    // init the sheet
    let mut sheet = Sheet::new();

    // add some values to the sheet
    sheet.write_cell(CellLoc { col: String::from("A"), row: (1) }, CellVal::Int(1));
    sheet.write_cell(CellLoc { col: String::from("A"), row: (3) }, CellVal::Int(3));
    sheet.write_cell(CellLoc { col: String::from("A"), row: (5) }, CellVal::Int(5));
    sheet.write_cell(CellLoc { col: String::from("B"), row: (1) }, CellVal::Int(1));
    sheet.write_cell(CellLoc { col: String::from("B"), row: (3) }, CellVal::Int(3));
    sheet.write_cell(CellLoc { col: String::from("B"), row: (5) }, CellVal::Int(5));
    sheet.write_cell(CellLoc { col: String::from("B"), row: (4) }, CellVal::Text(String::from("four")));
    //eprintln!("sheet: {:?}", sheet);

    // parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {  // ensure there is a subcommand
        eprintln!("no subcommand");
        process::exit(1);
    }
    let subcommand = &args[1];
    let other_args = &args[2..];
    handle_subcommand(subcommand, other_args, &mut sheet);
    
    //eprintln!("sheet: {:?}", sheet);
}
