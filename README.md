# MandelbrotViewer
Simple GUI for viewing the Mandelbrot-Set and Julia-Sets.

# if you just want to play around with the GUI:
- make sure you have python installed (3.9.2 or higher)
- run `non_rust/ui_main.py`

# if you are intrested in how this works:
- all images are rendered in Rust.
- `src/julia.rs` contains most of the interesting code (be warned it's not very well organized)
- `src/polar.rs` contains some utility functions for navigating the main cardiod of the Mandelbrot-Set without a GUI
---
- the GUI is written in Python with PyQt5
- check out `non_rust/ui_main.py` to see how it works
---
- run.sh is a conviniece script, that compiles the rust code, moves the compiled module into /non_rust and runs ui_main.py
