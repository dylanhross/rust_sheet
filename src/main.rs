/*  */


use std::env;
use std::process;
use std::mem;
use std::cmp;
use std::io::{self, BufRead};
use std::fs;
use std::path;


#[derive(Debug)]
enum CellVal {
    Int(i32),
    Real(f64),
    Text(String),
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


#[derive(Debug)]
struct CellLoc {
    col: String,
    row: usize,
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


#[derive(Debug)]
struct Cell{
    val: CellVal,
    loc: CellLoc,
}


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


fn parse_first_line (line: &String) -> (usize, usize) {
    // first line gets parsed as "<n_cols> <n_rows>"
    let mut buf_cols = String::new();
    let mut buf_rows = String::new();
    let mut space_flag = false;
    for c in line.chars() {
        if c != ' ' {
            if space_flag {
                buf_rows.push(c);
            } else {
                buf_cols.push(c);
            }
        } else {
            space_flag = true;
        }
    }
    (buf_cols.parse::<usize>().unwrap(), buf_rows.parse::<usize>().unwrap())
}


fn parse_line (line: &String) -> (CellLoc, CellVal) {
    // all other lines after the first are parsed as "<loc> <val>"
    let mut buf_loc = String::new();
    let mut buf_val = String::new();
    let mut space_flag = false;
    let mut paren_flag = false;
    for c in line.chars() {
        if c != ' ' {
            if space_flag {
                if paren_flag {
                    if c == ')' {
                        paren_flag = false;
                    } else {
                        if c != '"' {  // ignore quotes from Text(...) values 
                            buf_val.push(c);
                        }
                    }
                } else {
                    if c == '(' {
                        paren_flag = true;
                    }
                }
            } else {
                buf_loc.push(c);
            }
        } else {
            space_flag = true;
        }
    }
    (parse_loc(&buf_loc), parse_val(&buf_val))
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

    fn load_sheet (&mut self) {
        eprintln!("loading sheet state from file (sheet.txt)");
        // File sheet.txt must exist in the current path
        if let Ok(lines) = read_lines("./sheet.txt") {
            let mut n_cells: usize = 0;
            // Consumes the iterator, returns an (Optional) String
            for (i, line) in lines.enumerate() {
                if i == 0 {
                    // parse the first line as "<n_cols> <n_rows>"
                    if let Ok(line) = line {
                        let (n_cols, n_rows) = parse_first_line(&line);
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
                        let (loc, val) = parse_line(&line);
                        self.write_cell(loc, val);
                        n_cells += 1;
                    }
                }
            }
            eprintln!("loaded {} cells", n_cells);
        }
    }

    fn save_sheet (&self) {
        eprintln!("save sheet not implemented yet");
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

    fn read_sheet (&self) {
        // first print <n_cols> <n_rows>
        println!("{} {}", self.n_cols, self.n_rows);
        for col in &self.cols {
            for cell in col {
                println!("{}{} {:?}", cell.loc.col, cell.loc.row, cell.val);
            }
        }
    }

    fn read_cell (&self, loc: CellLoc) {
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

    fn delete_cell (&mut self, loc: CellLoc) -> bool {
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

    fn shrink (&mut self) -> bool {
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


fn handle_subcommand (subcommand: &String, other_args: &[String], sheet: &mut Sheet) -> bool {
    let n_other_args = other_args.len();
    let mut modified = false;
    match subcommand.as_str() {
        "read_sheet" => {
            eprintln!("subcommand: {}", subcommand);
            sheet.read_sheet();
        },
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
            modified = true;
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
        "delete_cell" => {
            if n_other_args != 1 {
                eprintln!("delete_cell subcommand takes 1 arg: <loc>");
                process::exit(1);
            }
            eprintln!("subcommand: {}", subcommand);
            let loc = parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
            modified = sheet.delete_cell(loc);
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
            modified = true;
        },
        "add_col" => {
            eprintln!("subcommand: {}", subcommand);
            // print the number of cols in the sheet to stdout
            eprintln!("cols before: {}", sheet.n_cols);
            sheet.add_col();
            eprintln!("cols after: {}", sheet.n_cols);
            modified = true;
        },
        "shrink" => {
            eprintln!("subcommand: {}", subcommand);
            modified = sheet.shrink();
        },
        _ => {
            eprintln!("unrecognized subcommand: {}", subcommand);
            process::exit(1);
        },
    }
    modified
}


fn main () {
    // init the sheet
    let mut sheet = Sheet::new();

    // load sheet state from file
    sheet.load_sheet();

    // parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {  // ensure there is a subcommand
        eprintln!("no subcommand");
        process::exit(1);
    }
    let subcommand = &args[1];
    let other_args = &args[2..];
    let modified = handle_subcommand(subcommand, other_args, &mut sheet);

    // save sheet state to file (only if it has been modified)
    if modified {
        sheet.save_sheet();
    }
}
