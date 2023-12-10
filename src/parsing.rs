/*
    Module with functions for parsing information from strings
*/


use std::process;

use crate::dtypes;


pub fn parse_loc (loc_arg: &String) -> dtypes::CellLoc {
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
    dtypes::CellLoc {
        col: buf_col,
        row: buf_row.parse::<usize>().unwrap(),
    }
}


pub fn parse_val (val_arg: &String) -> dtypes::CellVal {
    match val_arg.parse::<i32>() {  // try parse as int first
        Ok(val) => dtypes::CellVal::Int(val),
        _ => match val_arg.parse::<f64>() {  // try parse as real next
            Ok(val) => dtypes::CellVal::Real(val),
            _ => {
                match val_arg.chars().nth(0) {
                    Some(c) => {
                        if c == '=' {
                            dtypes::CellVal::Formula(val_arg.clone())  // formula
                        } else {
                            dtypes::CellVal::Text(val_arg.clone())  // otherwise parse as text
                        }
                    }
                    _ => dtypes::CellVal::Text(val_arg.clone())  // otherwise parse as text
                }
            } 
        }
    }
}


pub fn parse_first_line (line: &String) -> (usize, usize) {
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


pub fn parse_line (line: &String) -> (dtypes::CellLoc, dtypes::CellVal) {
    // all other lines after the first are parsed as "<loc> <val>"
    let mut buf_loc = String::new();
    let mut buf_val = String::new();
    let mut space_flag = false;
    let mut paren_flag = false;
    for c in line.chars() {
        if c != ' ' || space_flag {
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
            // the first space encountered should be the one between
            // the loc and the value, before it is encountered spaces
            // are ignored but once that first space is encountered and
            // space flag is set, then we need to not ignore spaces
            space_flag = true;
        }
    }
    (parse_loc(&buf_loc), parse_val(&buf_val))
}




pub fn parse_formula (_val: &String) -> () {
    // ?
    //let mut buf_loc = String::new();
    //let mut buf_val = String::new();
    //let mut space_flag = false;
    //let mut paren_flag = false;
    //for c in line.chars() {

    // make sure that any references to cell locations
    // evaluate to numeric type (Int or Real)? or do
    // this somewhere else?
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_val_int () {
        let cv = parse_val(&String::from("1"));
        assert!(matches!(cv, dtypes::CellVal::Int(_)), "failed to parse cell value as Int");
        // would need to modify parser to handle stripping whitespace for
        // the rest of these test cases to work
        /*let cv = parse_val(&String::from("1 "));
        assert!(matches!(cv, CellVal::Int(_)), "failed to parse cell value as Int");
        let cv = parse_val(&String::from(" 1"));
        assert!(matches!(cv, CellVal::Int(_)), "failed to parse cell value as Int");
        let cv = parse_val(&String::from(" 1 "));
        assert!(matches!(cv, CellVal::Int(_)), "failed to parse cell value as Int");*/
    }

    #[test]
    fn parse_val_real () {
        let cv = parse_val(&String::from("1."));
        assert!(matches!(cv, dtypes::CellVal::Real(_)), "failed to parse cell value as Real");
        let cv = parse_val(&String::from("1.234"));
        assert!(matches!(cv, dtypes::CellVal::Real(_)), "failed to parse cell value as Real");
        // would need to modify parser to handle stripping whitespace for
        // the rest of these test cases to work
        /*let cv = parse_val(&String::from("1.234 "));
        assert!(matches!(cv, dtypes::CellVal::Real(_)), "failed to parse cell value as Real");
        let cv = parse_val(&String::from(" 1.234"));
        assert!(matches!(cv, CellVal::Int(_)), "failed to parse cell value as int");
        let cv = parse_val(&String::from(" 1.234 "));
        assert!(matches!(cv, CellVal::Int(_)), "failed to parse cell value as int");*/
    }

    #[test]
    fn parse_val_text () {
        let cv = parse_val(&String::from("abc"));
        assert!(matches!(cv, dtypes::CellVal::Text(_)), "failed to parse cell value as Text");
    }

    #[test]
    fn parse_val_formula () {
        let cv = parse_val(&String::from("=C3+C5*2"));
        assert!(matches!(cv, dtypes::CellVal::Formula(_)), "failed to parse cell value as Formula");
    }

    #[test]
    fn parse_loc_ () {
        // TODO
        assert!(true);
    }
}


