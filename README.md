# MandelbrotViewer
Simple GUI for viewing the Mandelbrot-Set and Julia-Sets.

# if you just want to play around with the GUI:
- make sure you have python installed (3.9.2 or higher)
- run `non_rust/ui_main.py` or `non_rust/run.sh`
- you might also have to recompile the rust-module with `run.sh` or `$ cargo build --release`, I haven't tested on other machines yet.

# if you are intrested in how this works:
- all images are rendered in Rust.
- `src/julia.rs` contains most of the interesting code (be warned it's not very well organized)
- `src/lib.rs` contains all the PyO3 code. Most important is the PlotWindow struct. It controls what get's rendered and is the main accespoint for my Python Code
---
- the GUI is written in Python with PyQt5
- check out `non_rust/ui_main.py` to see how it works
---
- `run.sh` is a conviniece script, that compiles the rust code, moves the compiled module into /non_rust and runs ui_main.py
