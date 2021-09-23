mod julia;

use cpython::{PyResult, py_module_initializer, py_class};
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

const MANDEL_FILE: &str = &"./renders/mandel.png";
const JULIA_FILE: &str = &"./renders/julia.png";

pub const X_DIF: f64 = 2.1333;
pub const Y_DIF: f64 = 1.2;

fn float_to_big_decimal(f: f64) -> BigDecimal {
    BigDecimal::from_str(format!("{}", f).as_str()).unwrap()
}

py_class!(pub class PlotWindow |py| {

    data pixel_dim: (u32, u32);
    data x_min: BigDecimal;
    data x_max: BigDecimal;
    data x_dif: BigDecimal;
    data y_min: BigDecimal;
    data y_max: BigDecimal;
    data y_dif: BigDecimal;
    data julia: julia::Julia;

    def __new__(_cls, pixel_dim: (u32, u32)) -> PyResult<PlotWindow> {
        // creates Plotwindow at fully zoomed out view
        let x_max = float_to_big_decimal(X_DIF);
        let x_min = - float_to_big_decimal(X_DIF);
        let x_dif = x_max.clone() - x_min.clone();
        let y_max = float_to_big_decimal(Y_DIF);
        let y_min = - float_to_big_decimal(Y_DIF);
        let y_dif = y_max.clone() - y_min.clone();
        let julia = julia::Julia::new(BigDecimal::from_str("0.25").unwrap(), BigDecimal::zero());
        PlotWindow::create_instance(py, pixel_dim, x_min, x_max, x_dif, y_min, y_max, y_dif, julia)
    }

    def __repr__(&self) -> PyResult<String> {
        let dim = self.pixel_dim(py);
        Ok(format!("({}, {}): x=({}, {}) y=({}, {})", dim.0, dim.1, self.x_min(py), self.x_max(py), self.y_min(py), self.y_max(py)))
    }

    def zoom(&self, p: (f64, f64), factor: f64) -> PyResult<PlotWindow> {
        let p = pix_to_cords(p, self.pixel_dim(py).clone(), self.x_min(py).clone(), self.x_dif(py).clone(), self.y_min(py).clone(), self.y_dif(py).clone());
        let factor = float_to_big_decimal(factor);

        let new_x_dif = self.x_dif(py) * &factor;
        let new_y_dif = self.y_dif(py) * &factor;

        let mut new_x_min = &p.0 - &new_x_dif / 2;
        let mut new_x_max = &p.0 + &new_x_dif / 2;

        let mut new_y_min = &p.1 - &new_y_dif / 2;
        let mut new_y_max = &p.1 + &new_y_dif / 2;

        if factor < BigDecimal::from_str("1").unwrap() {
            // new window should be contained in old window
            // fit x
            if new_x_min < self.x_min(py).clone() {
                new_x_min = self.x_min(py).clone();
                new_x_max = &new_x_min + &new_x_dif;
            }else if new_x_max > self.x_max(py).clone() {
                new_x_max = self.x_max(py).clone();
                new_x_min = &new_x_max - &new_x_dif;
            }
            // fit y
            if new_y_min < self.y_min(py).clone() {
                new_y_min = self.y_min(py).clone();
                new_y_max = &new_y_min + &new_y_dif;
            }else if new_y_max > self.y_max(py).clone() {
                new_y_max = self.y_max(py).clone();
                new_y_min = &new_y_max - &new_y_dif;
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

fn pix_to_cords(p: (f64, f64), pix_dim: (u32, u32), x_min: BigDecimal, x_dif: BigDecimal, y_min: BigDecimal, y_dif: BigDecimal) -> (BigDecimal, BigDecimal) {
    let x = x_min + float_to_big_decimal(p.0 / pix_dim.0 as f64) * x_dif;
    let y = y_min + float_to_big_decimal(p.1 / pix_dim.1 as f64) * y_dif;
    (x, y)
}

// add bindings to the generated python module
// N.B: names: "mandelbrot" must be the name of the `.so` or `.pyd` file
py_module_initializer!(mandelbrot, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add_class::<PlotWindow>(py)?;
    Ok(())
});
