"""
    simple tkinter GUI for interactng with the spreadsheet engine
"""


from tkinter import *
from tkinter import ttk


class Backend:

    def __init__(self):
        pass


class App:

    def __init__(self):
        # initialize the window layout and widgets
        self._setup_win()
        self._setup_main_frm()
        self._setup_upper_lower_frm()
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
        self.main_frm.rowconfigure(0, weight=1)
        self.main_frm.rowconfigure(1, minsize=60, weight=0)
        self.main_frm.columnconfigure(0, weight=1)

    def _setup_upper_lower_frm(self):
        # upper frame layout
        self.upper_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.upper_frm.grid(row=0, column=0, sticky=(N, S, E, W))
        self.upper_frm.rowconfigure(0, weight=1)
        self.upper_frm.columnconfigure(0, weight=1)
        # upper frame widgets
        self.cvs = Canvas(self.upper_frm, relief=SUNKEN, borderwidth=2)
        self.cvs.grid(sticky=(N, S, E, W))
        # lower frame layout
        self.lower_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.lower_frm.grid(row=1, column=0, sticky=(N, S, E, W))
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

    def _setup_upper_lower_frm(self):
        # upper frame layout
        self.upper_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.upper_frm.grid(row=0, column=0, sticky=(N, S, E, W))
        self.upper_frm.rowconfigure(0, weight=1)
        self.upper_frm.columnconfigure(0, weight=1)
        # upper frame widgets
        self.cvs = Canvas(self.upper_frm, relief=SUNKEN, borderwidth=2)
        self.cvs.grid(sticky=(N, S, E, W))
        # lower frame layout
        self.lower_frm = ttk.Frame(self.main_frm, padding=(5, 5, 5, 5))
        self.lower_frm.grid(row=1, column=0, sticky=(N, S, E, W))
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

    def _txt_writeln(self, line):
        """ 
            method for writing lines to the textbox 

            the textbox is read-only from the front end, but the app
            will redirect all stderr messages from the backend into it
        """
        self.txt['state'] = NORMAL
        self.txt.insert(INSERT, '\n' + line)
        self.txt.see(END)
        self.txt['state'] = DISABLED


def main():
    app = App()


if __name__ == '__main__':
    main()
