/*  */


use std::env;
use std::process;


mod dtypes;
mod sheet;
mod parsing;
mod formulas;


fn handle_subcommand (subcommand: &String, other_args: &[String], sheet: &mut sheet::Sheet) -> bool {
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
            let loc = parsing::parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
            let val = parsing::parse_val(&other_args[1]);
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
            let loc = parsing::parse_loc(&other_args[0]);
            eprintln!("parsed cell location: {:?}", loc);
            sheet.read_cell(loc);
        },
        "delete_cell" => {
            if n_other_args != 1 {
                eprintln!("delete_cell subcommand takes 1 arg: <loc>");
                process::exit(1);
            }
            eprintln!("subcommand: {}", subcommand);
            let loc = parsing::parse_loc(&other_args[0]);
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
        "clear_sheet" => {
            eprintln!("subcommand: {}", subcommand);
            sheet.clear_sheet();
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
    let mut sheet = sheet::Sheet::new();

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
        eprintln!("sheet modified");
        sheet.read_sheet();
        sheet.save_sheet();
    }
}


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn some_main_test_ () {
        // TODO
        assert!(true);
    }
}


