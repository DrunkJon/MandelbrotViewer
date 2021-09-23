mod julia;

use cpython::{PyResult, Python, py_module_initializer, py_fn, py_class};

const MANDEL_FILE: &str = &"./renders/mandel.png";
const JULIA_FILE: &str = &"./renders/julia.png";

py_class!(pub class PlotWindow |py| {

    data pixel_dim: (u32, u32);
    data x_min: f64;
    data x_max: f64;
    data x_dif: f64;
    data y_min: f64;
    data y_max: f64;
    data y_dif: f64;
    data julia: julia::Julia;

    def __new__(_cls, pixel_dim: (u32, u32)) -> PyResult<PlotWindow> {
        // creates Plotwindow at fully zoomed out view
        let x_max = julia::X_DIF;
        let x_min = - julia::X_DIF;
        let x_dif = x_max - x_min;
        let y_max = julia::Y_DIF;
        let y_min = - julia::Y_DIF;
        let y_dif = y_max - y_min;
        let julia = julia::Julia::new(0.25, 0.0);
        PlotWindow::create_instance(py, pixel_dim, x_min, x_max, x_dif, y_min, y_max, y_dif, julia)
    }

    def __repr__(&self) -> PyResult<String> {
        let dim = self.pixel_dim(py);
        Ok(format!("({}, {}): x=({}, {}) y=({}, {})", dim.0, dim.1, self.x_min(py), self.x_max(py), self.y_min(py), self.y_max(py)))
    }

    // pix_to_cords is implemented through julia::convert_range

    def zoom(&self, p: (f64, f64), factor: f64) -> PyResult<PlotWindow> {
        let p = pix_to_cords(p, self.pixel_dim(py).clone(), self.x_min(py).clone(), self.x_dif(py).clone(), self.y_min(py).clone(), self.y_dif(py).clone());

        let new_x_dif = self.x_dif(py) * factor;
        let new_y_dif = self.y_dif(py) * factor;

        let mut new_x_min = p.0 - 0.5 * new_x_dif;
        let mut new_x_max = p.0 + 0.5 * new_x_dif;

        let mut new_y_min = p.1 - 0.5 * new_y_dif;
        let mut new_y_max = p.1 + 0.5 * new_y_dif;

        if factor < 1.0 {
            // new window should be contained in old window
            // fit x
            if new_x_min < self.x_min(py).clone() {
                new_x_min = self.x_min(py).clone();
                new_x_max = new_x_min + new_x_dif;
            }else if new_x_max > self.x_max(py).clone() {
                new_x_max = self.x_max(py).clone();
                new_x_min = new_x_max - new_x_dif;
            }
            // fit y
            if new_y_min < self.y_min(py).clone() {
                new_y_min = self.y_min(py).clone();
                new_y_max = new_y_min + new_y_dif;
            }else if new_y_max > self.y_max(py).clone() {
                new_y_max = self.y_max(py).clone();
                new_y_min = new_y_max - new_y_dif;
            }
        }

        PlotWindow::create_instance(py, self.pixel_dim(py).clone(), new_x_min, new_x_max, new_x_dif, new_y_min, new_y_max, new_y_dif, self.julia(py).clone())
    }

    def load_mandelbrot(&self, tries: u32) -> PyResult<String> {
        let dim = self.pixel_dim(py);
        julia::fine_mandelbrot(self.x_min(py).clone(), self.x_max(py).clone(), self.y_min(py).clone(), self.y_max(py).clone(), dim.0 , dim.1, MANDEL_FILE, tries);
        Ok(String::from(MANDEL_FILE))
    }

    def load_julia(&self, tries: u32) -> PyResult<String> {
        let dim = self.pixel_dim(py);
        let jul = self.julia(py).clone();
        julia::main_julia(jul, self.x_min(py).clone(), self.x_max(py).clone(), self.y_min(py).clone(), self.y_max(py).clone(), dim.0 , dim.1, JULIA_FILE, tries);
        Ok(String::from(JULIA_FILE))
    }

    def set_julia(&self, j_pix_cords: (f64, f64)) -> PyResult<PlotWindow> {
        let j_cords = pix_to_cords(j_pix_cords, self.pixel_dim(py).clone(), self.x_min(py).clone(), self.x_dif(py).clone(), self.y_min(py).clone(), self.y_dif(py).clone()); 
        let julia = julia::Julia::new(j_cords.0, j_cords.1);
        PlotWindow::create_instance(py,  self.pixel_dim(py).clone(), self.x_min(py).clone(), self.x_max(py).clone(), self.x_dif(py).clone(), self.y_min(py).clone(), self.y_max(py).clone(), self.y_dif(py).clone(), julia)
    }
});

fn pix_to_cords(p: (f64, f64), pix_dim: (u32, u32), x_min: f64, x_dif: f64, y_min: f64, y_dif: f64) -> (f64, f64) {
    let x = x_min + (p.0 / pix_dim.0 as f64) * x_dif;
    let y = y_min + (p.1 / pix_dim.1 as f64) * y_dif;
    (x, y)
}

// add bindings to the generated python module
// N.B: names: "mandelbrot" must be the name of the `.so` or `.pyd` file
py_module_initializer!(mandelbrot, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "julia", py_fn!(py, julia_py(jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32)))?;
    m.add(py, "raw_julia", py_fn!(py, raw_julia_py(jx: f64, jy: f64, scale: u32, tries: u32)))?;
    m.add(py, "mandelbrot", py_fn!(py, mandelbrot_py(scale: u32, out_file: &str, tries: u32)))?;
    m.add(py, "fine_julia", py_fn!(py, fine_julia_py(jx: f64, jy: f64, x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32)))?;
    m.add(py, "fine_mandelbrot", py_fn!(py, fine_mandelbrot_py(x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32)))?;
    m.add_class::<PlotWindow>(py)?;
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

fn fine_julia_py(_: Python, jx: f64, jy: f64, x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    let jul = julia::Julia::new(jx, jy);
    julia::main_julia(jul, x_min, x_max, y_min, y_max, 16 * scale , 9 * scale, out_file, tries);
    Ok(String::from(out_file))
}

fn fine_mandelbrot_py(_: Python, x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    julia::fine_mandelbrot(x_min, x_max, y_min, y_max, 16 * scale , 9 * scale, out_file, tries);
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
