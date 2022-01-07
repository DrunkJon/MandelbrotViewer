mod julia;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

const MANDEL_FILE: &str = &"./renders/mandel.png";
const JULIA_FILE: &str = &"./renders/julia.png";

#[pyclass]
pub struct PlotWindow {
    pixel_dim: (u32, u32),
    x_min: f64,
    x_max: f64,
    x_dif: f64,
    y_min: f64,
    y_max: f64,
    y_dif: f64,
    julia: julia::Julia
}

#[pymethods]
impl PlotWindow {
    #[new]
    fn __new__(pixel_dim: (u32, u32)) -> Self {
        // creates Plotwindow at fully zoomed out view
        let x_max = julia::X_DIF;
        let x_min = - julia::X_DIF;
        let x_dif = x_max - x_min;
        let y_max = julia::Y_DIF;
        let y_min = - julia::Y_DIF;
        let y_dif = y_max - y_min;
        let julia = julia::Julia::new(0.25, 0.0);
        PlotWindow {
            pixel_dim, x_min, x_max, x_dif, y_min, y_max, y_dif, julia
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        let dim = self.pixel_dim;
        Ok(format!("({}, {}): x=({}, {}) y=({}, {})", dim.0, dim.1, self.x_min, self.x_max, self.y_min, self.y_max))
    }

    // pix_to_cords is implemented through julia::convert_range

    fn zoom(&self, p: (f64, f64), factor: f64) -> PyResult<PlotWindow> {
        let p = pix_to_cords(p, self.pixel_dim.clone(), self.x_min.clone(), self.x_dif.clone(), self.y_min.clone(), self.y_dif.clone());

        let new_x_dif = self.x_dif * factor;
        let new_y_dif = self.y_dif * factor;

        let mut new_x_min = p.0 - 0.5 * new_x_dif;
        let mut new_x_max = p.0 + 0.5 * new_x_dif;

        let mut new_y_min = p.1 - 0.5 * new_y_dif;
        let mut new_y_max = p.1 + 0.5 * new_y_dif;

        if factor < 1.0 {
            // new window should be contained in old window
            // fit x
            if new_x_min < self.x_min.clone() {
                new_x_min = self.x_min.clone();
                new_x_max = new_x_min + new_x_dif;
            }else if new_x_max > self.x_max.clone() {
                new_x_max = self.x_max.clone();
                new_x_min = new_x_max - new_x_dif;
            }
            // fit y
            if new_y_min < self.y_min.clone() {
                new_y_min = self.y_min.clone();
                new_y_max = new_y_min + new_y_dif;
            }else if new_y_max > self.y_max.clone() {
                new_y_max = self.y_max.clone();
                new_y_min = new_y_max - new_y_dif;
            }
        }

        Ok(PlotWindow {
            pixel_dim: self.pixel_dim.clone(), 
            x_min: new_x_min, 
            x_max: new_x_max, 
            x_dif: new_x_dif, 
            y_min: new_y_min, 
            y_max: new_y_max, 
            y_dif: new_y_dif, 
            julia: self.julia.clone()
        })
    }

    fn load_mandelbrot(&self, tries: u32) -> PyResult<String> {
        let dim = self.pixel_dim;
        julia::fine_mandelbrot(self.x_min.clone(), self.x_max.clone(), self.y_min.clone(), self.y_max.clone(), dim.0 , dim.1, MANDEL_FILE, tries);
        Ok(String::from(MANDEL_FILE))
    }

    fn load_julia(&self, tries: u32) -> PyResult<String> {
        let dim = self.pixel_dim;
        let jul = self.julia.clone();
        julia::main_julia(jul, self.x_min.clone(), self.x_max.clone(), self.y_min.clone(), self.y_max.clone(), dim.0 , dim.1, JULIA_FILE, tries);
        Ok(String::from(JULIA_FILE))
    }

    fn set_julia(&self, j_pix_cords: (f64, f64)) -> PyResult<PlotWindow> {
        let j_cords = pix_to_cords(j_pix_cords, self.pixel_dim.clone(), self.x_min.clone(), self.x_dif.clone(), self.y_min.clone(), self.y_dif.clone()); 
        let julia = julia::Julia::new(j_cords.0, j_cords.1);
        Ok(PlotWindow {
            pixel_dim: self.pixel_dim.clone(), 
            x_min: self.x_min.clone(), 
            x_max: self.x_max.clone(), 
            x_dif: self.x_dif.clone(), 
            y_min: self.y_min.clone(), 
            y_max: self.y_max.clone(), 
            y_dif: self.y_dif.clone(), 
            julia
        })
    }
}

fn pix_to_cords(p: (f64, f64), pix_dim: (u32, u32), x_min: f64, x_dif: f64, y_min: f64, y_dif: f64) -> (f64, f64) {
    let x = x_min + (p.0 / pix_dim.0 as f64) * x_dif;
    let y = y_min + (p.1 / pix_dim.1 as f64) * y_dif;
    (x, y)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn mandelbrot_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(julia, m)?)?;
    m.add_function(wrap_pyfunction!(raw_julia, m)?)?;
    m.add_function(wrap_pyfunction!(mandelbrot, m)?)?;
    m.add_function(wrap_pyfunction!(fine_julia, m)?)?;
    m.add_function(wrap_pyfunction!(fine_mandelbrot, m)?)?;
    m.add_class::<PlotWindow>()?;
    Ok(())
}

#[pyfunction]
fn julia(jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    julia::single_julia(jx, jy, scale, out_file, tries);
    Ok(String::from(out_file))
}

#[pyfunction]
fn raw_julia(jx: f64, jy: f64, scale: u32, tries: u32) -> PyResult<Vec<u8>> {
    Ok(julia::raw_single_julia(jx, jy, scale, tries))
}

#[pyfunction]
fn mandelbrot(scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    julia::main_mandelbrot(scale, out_file, tries);
    Ok(String::from(out_file))
}

#[pyfunction]
fn fine_julia(jx: f64, jy: f64, x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
    let jul = julia::Julia::new(jx, jy);
    julia::main_julia(jul, x_min, x_max, y_min, y_max, 16 * scale , 9 * scale, out_file, tries);
    Ok(String::from(out_file))
}

#[pyfunction]
fn fine_mandelbrot(x_min: f64, x_max: f64, y_min: f64, y_max: f64, scale: u32, out_file: &str, tries: u32) -> PyResult<String> {
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
