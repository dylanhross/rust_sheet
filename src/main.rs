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

fn parse_loc(loc_arg: &String) -> CellLoc {
    println!("parse_loc: loc_arg: {}", loc_arg);
    let mut buf_col = String::new();
    let mut buf_row = String::new();
    for c in loc_arg.chars() {
        println!("{}", c);
        if c.is_alphabetic() {
            buf_col.push(c);
        }
        else if c.is_numeric() {
            buf_row.push(c);
        }
    }
    println!("buf_col: {}", buf_col);
    println!("buf_row: {}", buf_row);
    CellLoc {
        col: buf_col,
        row: buf_row.parse::<i32>().unwrap(),
    }
}

fn parse_val(val_arg: &String) -> CellVal {
    println!("parse_val: val_arg: {}", val_arg);
    CellVal::Int(val_arg.parse::<i32>().unwrap())
}

/* 
fn write_cell(loc: &CellLoc, val: &CellVal) {
}
*/


fn handle_subcommand(subcommand: &String, other_args: &[String]) {
    let n_other_args = other_args.len();
    match subcommand.as_str() {
        "write_cell" => {
            if n_other_args != 2 {
                eprintln!("write_cell subcommand takes 2 args: <loc> <value>");
                process::exit(1);
            }
            let loc = parse_loc(&other_args[0]);
            println!("parsed cell location: {:?}", loc);
            let val = parse_val(&other_args[1]);
            println!("parsed cell value: {:?}", val);
            //write_cell(&loc, &val);
        },
        "read_cell" => {
            if n_other_args != 1 {
                eprintln!("read_cell subcommand takes 1 arg: <loc>");
                process::exit(1);
            }
            println!("doing read cell ");
        },
        _ => {
            eprintln!("unrecognized subcommand: {}", subcommand);
            process::exit(1);
        },
    }
}


fn main() {

    // parse arguments
    let args: Vec<String> = env::args().collect();
    // ensure there is a subcommand
    if args.len() < 2 {
        eprintln!("no subcommand");
        process::exit(1);
    }
    
    let subcommand = &args[1];
    let other_args = &args[2..];
    handle_subcommand(subcommand, other_args);
    
}
