pub mod complex;
use complex::Complex;

pub mod polar;
pub mod colors;

use image::{RgbImage, Rgb, ImageBuffer};
use std::sync::mpsc;
use threadpool::ThreadPool;

pub trait Fractal {
    // function that returns how stable the given cords are for the fractal
    fn stable_cords(&self, x: f64, y: f64, tries:u32, power: u32) -> u32 {
        let con = Complex::new(x, y);
        self.stable(con, tries, power)
    }

    fn stable(&self, start: Complex, tries: u32, power: u32) -> u32;
}

pub type Julia = Complex;   //represents starting point

impl Fractal for Julia {
    fn stable(&self, start: Complex, tries: u32, power: u32) -> u32 {
        let mut z = start;
        for i in 0..tries {
            z = z.powi(power) + self.clone();
            if z.dist_from_origin() > 2.0 {
                return i;
            }
        }
        return tries;
    }
}

/*
// Problem: can't transfer PolyJulia between threads
struct PolyJulia {
    factors: Vec<Complex>
}

impl PolyJulia {

    pub fn new(factors: Vec<Complex>) -> Self {
        Self{factors: factors}
    }
}

impl Fractal for PolyJulia {
    fn stable(&self, start: Complex, tries: u32) -> u32 {
        let mut z = start;
        for i in 0..tries {
            for (j, c) in self.factors.iter().enumerate() {
                z += c.clone() * z.powi(j as u32);
            }
            // not sure if this is true for PolyJulias :/
            if z.dist_from_origin() > 2.0 {
                return i;
            }
        }
        return tries;
    }
} 
*/

pub fn mandelbrot(cx: f64, cy: f64, tries: u32, power: u32) -> u32 {
    let c = Complex::new(cx, cy);
    let mut z = Complex::null();
    for i in 0..tries {
        z = z.powi(power) + c;
        if z.dist_from_origin() > 2.0 {
            return i 
        }
    }
    tries
}

fn convert_range(min: f64, max: f64, slices: u32) -> polar::Iter<(u32, f64)> {
    let dif = max - min;
    Box::new(
        (0..slices).map(move |i| {
            (i, min + (i as f64 / (slices -1) as f64) * dif)
        })
    )
}

pub fn single_julia(jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32, power: u32) {
    let jul = Julia::new(jx, jy);
    main_julia(jul, -X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, out_file, tries, power)
}

pub fn raw_single_julia(jx: f64, jy: f64, scale: u32, tries: u32, power: u32) -> Vec<u8> {
    let jul = Julia::new(jx, jy);
    raw_julia(jul, -X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, tries, power)
}

pub const X_DIF: f64 = 2.1333;
pub const Y_DIF: f64 = 1.2;

pub fn main_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, out_file: &str, tries: u32, power: u32) {
    let img = render_julia(julia, x_min, x_max, y_min, y_max, x_range, y_range, tries, power);
    img.save(out_file).expect("could not save image");
}

pub fn raw_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32, power: u32) -> Vec<u8> {
    let img = render_julia(julia, x_min, x_max, y_min, y_max, x_range, y_range, tries, power);
    img.into_raw()
}

fn render_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32, power: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut recievers = Vec::new();

    let workers = 32; // 2 x cores on my PC
    let pool = ThreadPool::new(workers);
    
    for (_, cord_x) in convert_range(x_min, x_max, x_range) {
        let (tx, rx) = mpsc::channel();
        recievers.push(rx);
        pool.execute(move || {
            let mut line = Vec::new();
            
            for (_, cord_y) in convert_range(y_min, y_max, y_range) {
                let i = julia.stable_cords(cord_x, cord_y, tries, power);
                if i != tries {
                    line.push(colors::color_builder(i));
                } else {
                    line.push(Rgb([0, 0, 0]));
                }
            }
            
            tx.send(line).unwrap();
        });
    }
    
    for (x, rx) in recievers.into_iter().enumerate() {
        let colors = rx.recv().unwrap();
        for (y, c) in colors.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, c);
        }
    }
    pool.join();
    
    img
}

pub fn main_mandelbrot(scale: u32, out_file: &str, tries: u32, power: u32) {
    let img = render_mandelbrot(-X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, tries, power);
    img.save(out_file).expect("could not save image");
}

pub fn fine_mandelbrot(x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, out_file: &str, tries: u32, power: u32) {
    render_mandelbrot(x_min, x_max, y_min, y_max, x_range, y_range, tries, power).save(out_file).expect("could not save image")
}

fn render_mandelbrot(x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32, power: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut recievers = Vec::new();

    let workers = 32; // 2 x cores on my PC
    let pool = ThreadPool::new(workers);
    
    for (_, cord_x) in convert_range(x_min, x_max, x_range) {
        let (tx, rx) = mpsc::channel();
        recievers.push(rx);
        pool.execute(move || {
            let mut line = Vec::new();
            
            for (_, cord_y) in convert_range(y_min, y_max, y_range) {
                let i = mandelbrot(cord_x, cord_y, tries, power);
                if i != tries {
                    line.push(colors::color_builder(i));
                } else {
                    line.push(Rgb([0, 0, 0]));
                }
            }
            
            tx.send(line).unwrap();
        });
    }
    
    for (x, rx) in recievers.into_iter().enumerate() {
        let colors = rx.recv().unwrap();
        for (y, c) in colors.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, c);
        }
    }
    pool.join();
    
    img
}