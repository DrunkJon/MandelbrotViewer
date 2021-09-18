mod julia;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(mandelbrot, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "julia", py_fn!(py, julia_py(jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32)))?;
    m.add(py, "raw_julia", py_fn!(py, raw_julia_py(jx: f64, jy: f64, scale: u32, tries: u32)))?;
    m.add(py, "mandelbrot", py_fn!(py, mandelbrot_py(scale: u32, out_file: &str, tries: u32)))?;
    Ok(())
});

fn julia_py(_: Python, jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    julia::single_julia(jx, jy, scale, out_file, tries);
    Ok(String::from(out_file))
}

fn raw_julia_py(_: Python, jx: f64, jy: f64, scale: u32, tries: u32) -> PyResult<Vec<u8>> {
    Ok(julia::raw_single_julia(jx, jy, scale, tries))
}

fn mandelbrot_py(_: Python, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    julia::main_mandelbrot(scale, out_file, tries);
    Ok(String::from(out_file))
}

#[cfg(test)]
mod test{
    use super::julia;
    use std::fs::remove_file;
    use std::path::Path;

    #[test]
    fn single_julia_test() {
        julia::single_julia(0.25, 0.0, 60, "./test.png", 50);
        let path = Path::new("./test.png");
        remove_file(path).expect("could not delete test.png");
    }

    #[test]
    fn raw_julia_test() {
        let raw = julia::raw_single_julia(0.25, 0.0, 60, 50);
        assert_eq!(raw.is_empty(), false);
    }
}
