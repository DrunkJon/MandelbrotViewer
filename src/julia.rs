pub mod complex;
use complex::Complex;

use image::{RgbImage, Rgb, ImageBuffer};
use std::thread;
use std::sync::mpsc;

use bigdecimal::{BigDecimal, Zero};

pub type Julia = Complex;   //represents starting point

type Iter<T> = Box<dyn Iterator<Item = T>>;

impl Julia {

    pub fn stable(self, start: Complex, tries: u32) -> u32 {
        let mut z = start;
        for i in 0..tries {
            z = z.powi(2) + self.clone();
            if z.dist_from_origin() > 2.0 {
                return i;
            }
        }
        return tries;
    }

    pub fn stable_cords(self, x: BigDecimal, y: BigDecimal, tries:u32) -> u32 {
        let con = Complex::new(x, y);
        self.stable(con, tries)
    }
}

struct PolyJulia {
    factors: Vec<Complex>
}

impl PolyJulia {

    pub fn new(factors: Vec<Complex>) -> Self {
        Self{factors: factors}
    }

    pub fn stable(self, start: Complex, tries: u32) -> u32 {
        let mut z = start;
        for i in 0..tries {
            for (i, c) in self.factors.iter().enumerate() {
                let mut new_z = Complex::new(BigDecimal::zero(), BigDecimal::zero());
                z = c.clone() * z.powi(i as u32);
            }
        }
        unimplemented!()
    }
}

pub fn mandelbrot(cx: BigDecimal, cy: BigDecimal, tries: u32) -> u32 {
    let c = Complex::new(cx, cy);
    let mut z = Complex::null();
    for i in 0..tries {
        z = z.powi(2) + c.clone();
        if z.dist_from_origin() > 2.0 {
            return i 
        }
    }
    tries
} 

fn convert_range(min: BigDecimal, max: BigDecimal, slices: u32) -> Iter<(u32, BigDecimal)> {
    let dif = max - min.clone();
    Box::new(
        (0..slices).map(move |i| {
            (i, min.clone() + (BigDecimal::from(i) / BigDecimal::from(slices -1)) * dif.clone())
        })
    )
}

pub fn main_julia(julia: Julia, x_min: BigDecimal, x_max: BigDecimal, y_min:BigDecimal, y_max: BigDecimal, x_range: u32, y_range: u32, out_file: &str, tries: u32) {
    let img = render_julia(julia, x_min, x_max, y_min, y_max, x_range, y_range, tries);
    img.save(out_file).expect("could not save image");
}

fn render_julia(julia: Julia, x_min: BigDecimal, x_max: BigDecimal, y_min: BigDecimal, y_max: BigDecimal, x_range: u32, y_range: u32, tries: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut handles = Vec::new();
    
    for (_, cord_x) in convert_range(x_min.clone(), x_max.clone(), x_range) {
        let (tx, rx) = mpsc::channel();
        let y_min = y_min.clone();
        let y_max = y_max.clone();
        let julia = julia.clone();
        handles.push((rx, thread::spawn(move || {
            let mut line = Vec::new();
            
            for (_, cord_y) in convert_range(y_min.clone(), y_max.clone(), y_range) {
                let i = julia.clone().stable_cords(cord_x.clone(), cord_y, tries);
                if i != tries {
                    line.push(ratio_to_hue(i as f64 / tries as f64));
                } else {
                    line.push(Rgb([0, 0, 0]));
                }
            }
            
            tx.send(line).unwrap();
        })))
    }
    
    for (x, (rx, handle)) in handles.into_iter().enumerate() {
        let colors = rx.recv().unwrap();
        for (y, c) in colors.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, c);
        }
        handle.join().unwrap();
    }
    
    img
}

pub fn fine_mandelbrot(x_min: BigDecimal, x_max: BigDecimal, y_min: BigDecimal, y_max: BigDecimal, x_range: u32, y_range: u32, out_file: &str, tries: u32) {
    render_mandelbrot(x_min, x_max, y_min, y_max, x_range, y_range, tries).save(out_file).expect("could not save image")
}

fn render_mandelbrot(x_min: BigDecimal, x_max: BigDecimal, y_min: BigDecimal, y_max: BigDecimal, x_range: u32, y_range: u32, tries: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut handles = Vec::new();
    
    for (_, cord_x) in convert_range(x_min, x_max, x_range) {
        let (tx, rx) = mpsc::channel();
        let y_min = y_min.clone();
        let y_max = y_max.clone();
        handles.push((rx, thread::spawn(move || {
            let mut line = Vec::new();
            
            for (_, cord_y) in convert_range(y_min.clone(), y_max.clone(), y_range) {
                let i = mandelbrot(cord_x.clone(), cord_y, tries);
                if i != tries {
                    line.push(ratio_to_hue(i as f64 / tries as f64));
                } else {
                    line.push(Rgb([0, 0, 0]));
                }
            }
            
            tx.send(line).unwrap();
        })))
    }
    
    for (x, (rx, handle)) in handles.into_iter().enumerate() {
        let colors = rx.recv().unwrap();
        for (y, c) in colors.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, c);
        }
        handle.join().unwrap();
    }
    
    img
}

const C_LOW: u8 = 55;
const C_HIGH: u8 = 255 - C_LOW;

fn ratio_to_hue(ratio: f64) -> Rgb<u8> {
    let ratio = ratio.sqrt();
    let mut r = C_LOW;
    let mut g = C_LOW;
    let mut b = C_LOW;
    if ratio < 0.5 {
        let ratio = ratio * 2.0;
        g += (C_HIGH as f64 * ratio + 0.5) as u8;
        b += (C_HIGH as f64 * (1.0 - ratio) + 0.5) as u8;
        Rgb([r, g, b])
    } else if ratio <= 1.0{
        let ratio = (1.0 - ratio) * 2.0;
        g += (C_HIGH as f64 * ratio + 0.5) as u8;
        r += (C_HIGH as f64 * (1.0 - ratio) + 0.5) as u8;
        Rgb([r, g, b])
    } else {
        panic!("tried to get hue for ratio > 1.0")
    }
}