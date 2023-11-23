# Overview
Try to learn rust by writing a basic spreadsheet engine: `rustsheet` 
## Features
- single sheet (no multi-sheet workbooks)
- data stored column-wise in vectors of cells
	- number of columns/rows grows and shrinks as data are added and removed
	- columns grow or shrink by adding new or taking away column vectors
	- rows grow or shrink by increasing or decreasing the length of all column vectors
	- all column vectors are maintained at the same length and grow/shrink together
	- cells each hold a single value with one of a few possible data types
		- int (`i32`)
		- float (`f64`)
		- string (`&str`) → a fun addition would be two separate string cell types where one is just a string and the other is a formula string that can operate on other cells
		- null (no value)
- sheet state is stored in a json file managed by this program (`sheet.json`)
- command-line interface for modifying state
	-  call signature something like `rustsheet <subcommand> args ...`
	- `<subcommand>` could be one of a few options
		- `read_cell <loc>` - print the value of a cell at `<loc>` to stdout
		- `write_cell <loc> <value>` -  write `<value>` into a cell at `<loc>`
		- `count_rows`
		- `count_cols`
		- `add_row` - add a new row to the sheet
		- `add_col` - a new column to the sheet
		- `shrink` - reduce the number of rows and columns in the sheet to the minimum amount necessary to hold all non-null cells
	- subcommands that return information (_e.g._ `read_cell`, `count_rows`) just print the result to stdout, others (_e.g._ `write_cell`, `add_row`) just modify the sheet state (in `sheet.json`)
	- indexing (`<loc>`) is in typical `<upper_letter><number>` format where the letter portion denotes the column (A, B, C, ...) and the number denotes the row (1, 2, 3, ...)
## sheet state
sheet state stored in `sheet.json` with the following layout
```json
{
	// store current extent of sheet
	// some columns/rows may be full of null cells
	n_cols: 4,
	n_rows: 20,
	// store non-null cell data
	// key: location
	// value: cell value (type inferred from value)
	// ordered by column, row (A1, A2, A3, B1, B2, ...)
	cells: {
		"A1": 123,
		"B2": 1.23,
		"C3": "one two three",
		// ...
		//
		// all cell locations within defined range without an explicit
		// value are implicitly considered null
	},
}
```
