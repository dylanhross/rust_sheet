"""
    simple tkinter GUI for interactng with the spreadsheet engine
"""


from tkinter import *
from tkinter import ttk
import subprocess
import re


class Backend:
    """ 
    interface for interacting with the rust_sheet spreadsheet 
    engine via a CLI
    """

    def __init__(self, stderr_cb):
        # sheet state is stored in 2D list shape: (columns, rows)
        # empty cells stored as None
        # cells with values stored as dict: {'t' cell type, 'v': cell value}
        self.sheet = [[]]
        self.n_cols = 0
        self.n_rows = 0
        self._exe = './target/debug/rust_sheet'
        # use this callback (already with the <BACKEND> prefix) to disbatch any
        # messages from the backend that are issued to stderr
        self._stderr_cb = lambda msg: stderr_cb('BACKEND', msg)
        self._val_pat = re.compile(r'(?P<t>[A-Za-z]+)[(](?P<v>.+)[)]')

    def _run(self, subcommand, other_args=[]):
        """ 
        run a subcommand, 
        dispatch any stderr messages to the front end via callback,
        return stdout as list of lines """
        cmd = [self._exe, subcommand] + other_args
        result = subprocess.run(cmd, capture_output=True, text=True)
        for line in result.stderr.splitlines():
            self._stderr_cb(line)
        return result.stdout.splitlines()

    def _fill_sheet_with_nones(self):
        """ 
        fills self.sheet with Nones so that it has the proper 
        number of columns and rows 
        """
        # clear the sheet out first
        self.sheet = []
        for i in range(self.n_cols):
            self.sheet.append([None for _ in range(self.n_rows)])

    def _col_to_idx(self, col):
        idx = 0
        for i, c in enumerate(col[::-1].encode()):
            idx += 26**i * (c - 64)
        return idx - 1
    
    def idx_to_col(self, idx):
        # This will get bad after a while lol
        # need to figure out the logic to go back to column names
        return "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[idx]

    def _parse_loc(self, loc):
        """ returns (column, row) both as indices into self.sheet """
        buf_col = ""
        buf_row = ""
        for c in loc:
            if c.isalpha():
                buf_col += c
            else:
                buf_row += c
        return self._col_to_idx(buf_col), int(buf_row) - 1
    
    def _parse_val(self, val):
        """ returns {'t': cell_type, 'v': cell_value} """
        parsed = self._val_pat.match(val).groupdict()
        if parsed['t'] == 'Int':
            parsed['v'] = int(parsed['v'])
        elif parsed['t'] == 'Real':
            parsed['v'] = float(parsed['v'])
        else:
            parsed['v'] = parsed['v'].strip('"')
        return parsed

    def _update_sheet(self, stdout_lines):
        """ update the sheet """
        nc_nr, *cell_lines = stdout_lines
        self.n_cols, self.n_rows = [int(_) for _ in nc_nr.split()]
        self._fill_sheet_with_nones()
        for cell_line in cell_lines:
            loc, val = cell_line.split(maxsplit=1)
            col, row = self._parse_loc(loc)
            val = self._parse_val(val)
            self.sheet[col][row] = val

    def read_sheet(self):
        """
        loads the sheet state via the read_sheet subcommand
        """
        self._update_sheet(self._run('read_sheet'))

    def clear_sheet(self):
        """
        clear the sheet
        """
        self._update_sheet(self._run('clear_sheet'))

    def add_col(self):
        """
        add a column to the sheet
        """
        self._update_sheet(self._run('add_col'))

    def add_row(self):
        """
        add a row to the sheet
        """
        self._update_sheet(self._run('add_row'))

    def shrink(self):
        """
        shrink the spreadsheet to minimal dimensions to contain all non-null cells
        """
        self._update_sheet(self._run('shrink'))

    def write_cell(self, col, row, val):
        """
        write a value into the cell
        """
        loc = "{}{}".format(self.idx_to_col(col), row + 1)
        self._update_sheet(self._run('write_cell', other_args=[loc, val]))


class GUI:
    """
    tkinter GUI app
    """

    def __init__(self):
        # init the window layout and widgets
        self._setup_win()
        self._setup_main_frm()
        self._setup_upper_frm()
        #self._setup_mid_frm()
        self._setup_lower_frm()
        # init the backend 
        self._backend = Backend(self._txt_writeln)
        self._backend.read_sheet()
        # draw the cells
        self._setup_mid_frm()
        # start up the window main loop
        self.win.mainloop()

    def _setup_win(self):
        # init window
        self.win = Tk()
        self.win.title('rust sheet')
        self.win.minsize(600, 300)
        # set up window layout cols and rows
        self.win.rowconfigure(0, weight=1)
        self.win.columnconfigure(0, weight=1)
    
    def _setup_main_frm(self):
        # init the main frame that will contain all other widgets
        self.main_frm = ttk.Frame(self.win, padding=(5, 5, 5, 5))
        self.main_frm.grid(row=0, column=0, sticky=(N, S, E, W))
        # set up the main frame's grid structure
        self.main_frm.rowconfigure(0, minsize=10, weight=0)
        self.main_frm.rowconfigure(1, weight=1)
        self.main_frm.rowconfigure(2, minsize=60, weight=0)
        self.main_frm.columnconfigure(0, weight=1)

    def _setup_upper_frm(self):
        # upper frame layout
        self.upper_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.upper_frm.grid(row=0, column=0, sticky=(N, S, E, W))
        self.upper_frm.rowconfigure(0, weight=1)
        self.upper_frm.columnconfigure(0, weight=1)
        # upper frame widgets
        # clear button
        self._setup_clr_btn()

    def _setup_clr_btn(self):
        self.clr_btn = ttk.Button(self.upper_frm, text='Clear Sheet', command=self._clr_btn_cb)
        self.clr_btn.pack(anchor=E)

    def _clr_btn_cb(self):
        """ callback for the Clear Sheet button """
        self._txt_writeln('GUI', 'hit the clear sheet button')
        self._backend.clear_sheet()
        self._draw_sheet()

    def _setup_mid_frm(self):
        # mid frame layout
        self.mid_frm = Frame(self.main_frm, borderwidth=2, relief=SUNKEN, padx=10, pady=10)
        self.mid_frm.grid(row=1, column=0, sticky=(N, S, E, W))
        self.mid_frm.rowconfigure(0, weight=0)
        for i in range(self._backend.n_rows):
            self.mid_frm.rowconfigure(i + 1, weight=1)
        self.mid_frm.rowconfigure(self._backend.n_rows + 2, weight=0)
        self.mid_frm.columnconfigure(0, weight=0, minsize=10)
        for i in range(self._backend.n_cols):
            self.mid_frm.columnconfigure(i + 1, weight=1)
        self.mid_frm.columnconfigure(self._backend.n_cols + 2, weight=0, minsize=10)
        # mid frame widgets
        self._setup_cell_txts()
        self._setup_addcol_btn()
        self._setup_addrow_btn()
        self._setup_shrink_btn()

    def _setup_cell_txts(self):
        self.cell_txts = []
        self.col_labs = []
        self.row_labs = []
        for col in range(self._backend.n_cols):
            self.col_labs.append(Label(self.mid_frm, text='{}'.format(self._backend.idx_to_col(col)), anchor=CENTER))
            self.col_labs[col].grid(row=0, column=col + 1, sticky=(E, W, S))
            self.cell_txts.append([])
            for row in range(self._backend.n_rows):
                self.cell_txts[col].append(Text(self.mid_frm, borderwidth=2, height=1, width=10, relief=SUNKEN))
                self.cell_txts[col][row].grid(row=row + 1, column=col + 1, sticky=(N, S, E, W))
                def update_handler(event, self=self, col=col, row=row):
                    return self._check_update_cell(event, col, row)
                self.cell_txts[col][row].bind('<FocusOut>', update_handler)
                cell = self._backend.sheet[col][row]
                if cell is not None:
                    self.cell_txts[col][row].insert(INSERT, cell["v"])
        for row in range(self._backend.n_rows):
            self.row_labs.append(Label(self.mid_frm, text='{}'.format(row + 1), anchor=CENTER))
            self.row_labs[row].grid(row=row + 1, column=0, sticky=(E,))
        # set up all of the navigation binds
        self._setup_nav_binds()

    def _setup_nav_binds(self):
        for col in range(self._backend.n_cols):
            for row in range(self._backend.n_rows):
                # bind arrow keys for navigation around the sheet
                # UP
                def kp_up_handler(event, col=col, row=row):
                    if row > 0:
                        self.cell_txts[col][row - 1].focus_set()
                self.cell_txts[col][row].bind('<KeyPress-Up>', kp_up_handler)
                # DOWN 
                def kp_down_handler(event, col=col, row=row):
                    if row < self._backend.n_rows - 1:
                        self.cell_txts[col][row + 1].focus_set()
                self.cell_txts[col][row].bind('<KeyPress-Down>', kp_down_handler)
                # LEFT
                def kp_left_handler(event, col=col, row=row):
                    if col > 0:
                        self.cell_txts[col - 1][row].focus_set()
                self.cell_txts[col][row].bind('<KeyPress-Left>', kp_left_handler)
                # RIGHT
                def kp_right_handler(event, col=col, row=row):
                    if col < self._backend.n_cols - 1:
                        self.cell_txts[col + 1][row].focus_set()
                self.cell_txts[col][row].bind('<KeyPress-Right>', kp_right_handler)

    def _check_update_cell(self, event, col, row):
        """ check the contents of a cell an update the sheet if its contents have changed """
        self._txt_writeln("GUI", "checking cell {}{}".format(self._backend.idx_to_col(col), row + 1))
        if self.cell_txts[col][row].edit_modified():
            current_val = self.cell_txts[col][row].get('1.0', END).strip()
            bkend_val = self._backend.sheet[col][row]
            if bkend_val is None:
                if current_val != "":
                    # update the cell on the backend
                    self._update_cell_bkend(col, row, current_val)
            else:
                if current_val != str(bkend_val["v"]):
                    # update cell on the backend
                    self._update_cell_bkend(col, row, current_val)
            # update the cell (trimming any unnecessary whitespace)
            self.cell_txts[col][row].delete('1.0', END)
            self.cell_txts[col][row].insert(INSERT, current_val)

    def _update_cell_bkend(self, col, row, current_val):
        self._txt_writeln("GUI", "updating cell ({}, {})".format(col, row))
        self._backend.write_cell(col, row, current_val)

    def _setup_addcol_btn(self):
        self.addcol_btn = ttk.Button(self.mid_frm, text='Add Column', command=self._addcol_btn_cb)
        self.addcol_btn.grid(row=0, column=self._backend.n_cols + 2, sticky=(E,))

    def _addcol_btn_cb(self):
        """ callback for the add column button """
        self._txt_writeln('GUI', 'hit the add column button')
        self._backend.add_col()
        self._draw_sheet()

    def _setup_addrow_btn(self):
        self.addrow_btn = ttk.Button(self.mid_frm, text='Add Row', command=self._addrow_btn_cb)
        self.addrow_btn.grid(row=self._backend.n_rows + 2, column=0, sticky=(W, E))

    def _addrow_btn_cb(self):
        """ callback for the add row button """
        self._txt_writeln('GUI', 'hit the add row button')
        self._backend.add_row()
        self._draw_sheet()

    def _setup_shrink_btn(self):
        self.shrink_btn = ttk.Button(self.mid_frm, text='Shrink', command=self._shrink_btn_cb)
        self.shrink_btn.grid(row=self._backend.n_rows + 2, column=self._backend.n_cols + 2, sticky=(W,))

    def _shrink_btn_cb(self):
        """ callback for the add column button """
        self._txt_writeln('GUI', 'hit the shrink button')
        self._backend.shrink()
        self._draw_sheet()

    def _setup_lower_frm(self):
        # lower frame layout
        self.lower_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.lower_frm.grid(row=2, column=0, sticky=(N, S, E, W))
        self.lower_frm.rowconfigure(0, weight=1)
        self.lower_frm.columnconfigure(0, weight=1)
        self.lower_frm.columnconfigure(1, minsize=5)
        # lower frame widgets
        self.txt_scrl = ttk.Scrollbar(self.lower_frm, orient='vertical')
        self.txt_scrl.grid(row=0, column=1, sticky=(N, S))
        self.txt = Text(self.lower_frm, relief=SUNKEN, borderwidth=2, height=5, 
                        highlightthickness=0, state=DISABLED, yscrollcommand=self.txt_scrl.set)
        self.txt.grid(row=0, column=0, sticky=(N, S, E, W))
        self.txt_scrl.config(command=self.txt.yview)

    def _txt_writeln(self, prefix, line):
        """ 
        method for writing lines to the textbox 

        the textbox is read-only from the front end, but the app
        will redirect all stderr messages from the backend into it
        """
        self.txt['state'] = NORMAL
        self.txt.insert(INSERT, '\n' + '<{}> '.format(prefix) + line)
        self.txt.see(END)
        self.txt['state'] = DISABLED

    def _draw_sheet(self):
        """ 
        draw sheet (stored in the backend) into the canvas
        actually just an alias for the _setup_mid_frm method 
        """
        self._setup_mid_frm()


def main():
    app = GUI()


if __name__ == '__main__':
    main()
