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
		- Int (`i32`)
		- Real (`f64`)
		- Text (`String`) → a fun addition would be two separate string cell types where one is just a string and the other is a formula string that can operate on other cells
		- ~~null (no value)~~ no need to explicitly store null cells, just store cells with actual values
- sheet state is stored in a json file managed by this program (`sheet.json`)
- command-line interface for modifying state
	-  call signature something like `rustsheet <subcommand> args ...`
	- `<subcommand>` could be one of a few options
		- `read_cell <loc>` - print the value of a cell at `<loc>` to stdout
		- `write_cell <loc> <value>` -  write `<value>` into a cell at `<loc>`, prints updated sheet to stdout
		- `read_sheet` - print the entire contents of the sheet to stdout, first line is "<n_cols>, <n_rows>" then each line after that is "<loc> <value>" for all non-null cells ordered by column, row
		- `count_rows` - print the number of rows in the sheet to stdout
		- `count_cols` - print the number of columns in the sheet to stdout
		- `add_row` - add a new row to the sheet (just increases the `n_rows` attribute of `Sheet`), prints new number of rows to stdout
		- `add_col` - a new column to the sheet, prints new number of columns to stdout
		- `shrink` - reduce the number of rows and columns in the sheet to the minimum amount necessary to hold all non-null cells, prints new dimensions as "<n_cols> <n_rows>" to stdout
		- `delete_cell <loc>` - delete the cell (if any) at `loc`, prints the updated sheet to stdout
	- subcommands that return information (_e.g._ `read_cell`, `count_rows`) just print the result to stdout, others (_e.g._ `write_cell`, `add_row`) just modify the sheet state (in `sheet.json`)
	- indexing (`<loc>`) is in typical `<upper_letter><number>` format where the letter portion denotes the column (A, B, C, ...) and the number denotes the row (1, 2, 3, ...)
- _BONUS_: write up a simple Python/tkinter GUI app for viewing/interacting with the spreadsheet engine (via CLI on the backend)

## sheet state
~~sheet state stored in `sheet.json` with the following layout`~~

Instead of using json just use a simpler text format (in `sheet.txt`) for now with the following layout
```
<n_cols> <n_rows>
<cell_loc> <cell_val>
<cell_loc> <cell_val>
... ...
```
This is the same formatting as what is printed to stdout when the `read_sheet` subcommand is used. 
