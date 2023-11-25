/*  */


use std::env;
use std::process;


#[derive(Debug)]
enum CellVal {
    Int(i32),
    Real(f64),
    Text(String),
}

#[derive(Debug)]
struct CellLoc {
    col: String,
    row: i32,
}


#[derive(Debug)]
struct Cell{
    val: CellVal,
    loc: CellLoc,
}


#[derive(Debug)]
struct Sheet {
    cols: Vec<Vec<Cell>>,
    n_cols: u32,
    n_rows: u32,
}

impl Sheet {
    fn new () -> Sheet {
        let cols: Vec<Vec<Cell>> = Vec::new();
        let sheet = Sheet {
            cols: cols,
            n_cols: 0,
            n_rows: 0,
        };
        sheet
    }

}


fn parse_loc(loc_arg: &String) -> CellLoc {
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
        row: buf_row.parse::<i32>().unwrap(),
    }
}


fn parse_val(val_arg: &String) -> CellVal {
    match val_arg.parse::<i32>() {  // try parse as int first
        Ok(val) => CellVal::Int(val),
        _ => match val_arg.parse::<f64>() {  // try parse as real next
            Ok(val) => CellVal::Real(val),
            _ => CellVal::Text(val_arg.clone())  // otherwise parse as text
        }
    }
}


fn write_cell(sht: &mut Sheet, loc: CellLoc, val: CellVal) {
    eprintln!("<write_cell>");
}


fn handle_subcommand(subcommand: &String, other_args: &[String], sheet: &mut Sheet) {
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
            write_cell(sheet, loc, val);
        },
        "read_cell" => {
            if n_other_args != 1 {
                eprintln!("read_cell subcommand takes 1 arg: <loc>");
                process::exit(1);
            }
            let loc = parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
        },
        _ => {
            eprintln!("unrecognized subcommand: {}", subcommand);
            process::exit(1);
        },
    }
}


fn main() {

    // init the sheet
    let mut sheet = Sheet::new();
    eprintln!("sheet: {:?}", sheet);

    // parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {  // ensure there is a subcommand
        eprintln!("no subcommand");
        process::exit(1);
    }
    let subcommand = &args[1];
    let other_args = &args[2..];
    handle_subcommand(subcommand, other_args, &mut sheet);
    
    eprintln!("sheet: {:?}", sheet);
}
