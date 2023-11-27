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
        return idx

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
        

class GUI:
    """
    tkinter GUI app
    """

    def __init__(self):
        # init the window layout and widgets
        self._setup_win()
        self._setup_main_frm()
        self._setup_upper_frm()
        self._setup_mid_frm()
        self._setup_lower_frm()
        # init the backend 
        self._backend = Backend(self._txt_writeln)
        self._backend.read_sheet()
        # draw the sheet
        self._draw_sheet()
        # start up the window main loop
        self.win.mainloop()

    def _setup_win(self):
        # init window
        self.win = Tk()
        self.win.title('rust sheet')
        self.win.minsize(600, 600)
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

    def _setup_mid_frm(self):
        # mid frame layout
        self.mid_frm_rows = 20
        self.mid_frm_cols = 4
        self.mid_frm = Frame(self.main_frm, borderwidth=3, relief=RIDGE, padx=10, pady=10)
        self.mid_frm.grid(row=1, column=0, sticky=(N, S, E, W))
        self.mid_frm.rowconfigure(0, weight=0)
        for i in range(self.mid_frm_rows):
            self.mid_frm.rowconfigure(i + 1, weight=1)
        self.mid_frm.rowconfigure(self.mid_frm_rows + 2, weight=0)
        self.mid_frm.columnconfigure(0, weight=0, minsize=10)
        for i in range(self.mid_frm_cols):
            self.mid_frm.columnconfigure(i + 1, weight=1)
        self.mid_frm.columnconfigure(self.mid_frm_cols + 2, weight=0, minsize=10)
        # mid frame widgets
        self._setup_cell_txts()
        self._setup_addcol_btn()
        self._setup_addrow_btn()
        self._setup_shrink_btn()

    def _setup_cell_txts(self):
        self.cell_txts = []
        self.col_labs = []
        self.row_labs = []
        for col in range(self.mid_frm_cols):
            self.col_labs.append(Label(self.mid_frm, text='{}'.format(col + 1), anchor=CENTER))
            self.col_labs[col].grid(row=0, column=col + 1, sticky=(E, W, S))
            self.cell_txts.append([])
            for row in range(self.mid_frm_rows):
                self.cell_txts[col].append(Label(self.mid_frm, borderwidth=2, anchor=CENTER, relief=RAISED))
                self.cell_txts[col][row].grid(row=row + 1, column=col + 1, sticky=(N, S, E, W))
        for row in range(self.mid_frm_rows):
            self.row_labs.append(Label(self.mid_frm, text='{}'.format(row + 1), anchor=CENTER))
            self.row_labs[row].grid(row=row + 1, column=0, sticky=(E,))

    def _setup_addcol_btn(self):
        self.addcol_btn = ttk.Button(self.mid_frm, text='Add Column', command=self._addcol_btn_cb)
        self.addcol_btn.grid(row=0, column=self.mid_frm_cols + 2, sticky=(E,))

    def _addcol_btn_cb(self):
        """ callback for the add column button """
        self._txt_writeln('GUI', 'hit the add column button')

    def _setup_addrow_btn(self):
        self.addrow_btn = ttk.Button(self.mid_frm, text='Add Row', command=self._addrow_btn_cb)
        self.addrow_btn.grid(row=self.mid_frm_rows + 2, column=0, sticky=(W, E))

    def _addrow_btn_cb(self):
        """ callback for the add row button """
        self._txt_writeln('GUI', 'hit the add row button')

    def _setup_shrink_btn(self):
        self.shrink_btn = ttk.Button(self.mid_frm, text='Shrink', command=self._shrink_btn_cb)
        self.shrink_btn.grid(row=self.mid_frm_rows + 2, column=self.mid_frm_cols + 2, sticky=(W,))

    def _shrink_btn_cb(self):
        """ callback for the add column button """
        self._txt_writeln('GUI', 'hit the shrink button')

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
        """ draw sheet (stored in the backend) into the canvas """


def main():
    app = GUI()


if __name__ == '__main__':
    main()
