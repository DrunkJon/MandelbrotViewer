pub mod complex;
use complex::Complex;

pub mod polar;

use image::{RgbImage, Rgb, ImageBuffer};
use pbr::ProgressBar;
use std::f64::consts::PI;
use std::fs::create_dir;
use std::path::Path;
use std::thread;
use std::sync::mpsc;

pub type Julia = Complex;   //represents starting point

impl Julia {

    pub fn stable(self, start: Complex, tries: u32) -> u32 {
        let mut z = start;
        for i in 0..tries {
            z = z * z + self;
            if z.dist_from_origin() > 2.0 {
                return i;
            }
        }
        return tries;
    }

    pub fn stable_cords(self, x: f64, y: f64, tries:u32) -> u32 {
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
                let mut new_z = Complex::new(0.0, 0.0);
                z = c.clone() * z.powi(i as u32);
            }
        }
        unimplemented!()
    }
}

pub fn mandelbrot(cx: f64, cy: f64, tries: u32) -> u32 {
    let c = Complex::new(cx, cy);
    let mut z = Complex::null();
    for i in 0..tries {
        z = z * z + c;
        if z.dist_from_origin() > 2.0 {
            return i 
        }
    }
    tries
} 

const RENDER_FOLDER: &str = "C:/Users/User/Desktop/plots";

fn convert_range(min: f64, max: f64, slices: u32) -> polar::Iter<(u32, f64)> {
    let dif = max - min;
    Box::new(
        (0..slices).map(move |i| {
            (i, min + (i as f64 / (slices -1) as f64) * dif)
        })
    )
}

pub fn main() {
    let a = PI / 4.0;
    let r_min = polar::cardioid(a, 0.5);
    let length = 0.05;
    let slices = 30;
    let folder_name = "radial_julia3b";
    create_dir(format!("{}/{}", RENDER_FOLDER, folder_name).as_str()).expect("folder already exists");
    println!("{}", r_min);
    for (i, (x, y)) in polar::rectangular_radial_path(a, r_min, r_min + length, slices).enumerate() {
        let x = x + 0.25;
        println!{"x:{} y:{}", x, y};
        single_julia(x, y, 180, format!("{}/{}/{}.png", RENDER_FOLDER, folder_name, i).as_str(), 250);
    }
}

fn configure_cardioid_pan_threaded() {
    let out_folder: &str = "plots/sj-t2";
    let p = Path::new(out_folder);
    create_dir(p).expect("folder already exists");
    let x_fac = 16;
    let y_fac = 9;
    let scale = 180;
    let slices = 720;
    let offset = 0.0;
    let angle = 2.0 * PI;
    let depth = 250;
    let derailment = 0.002;
    let prefix = "";
    let mut pb = ProgressBar::new(slices as u64);
    pb.format("|=+_|");
    for (i, (x, y)) in polar::main_cardioid_path(slices, offset, angle, derailment).enumerate() {
        let j = Julia::new(x, y);
        let out_file = String::from(out_folder) + &format!("/{}{}({},{}).png", prefix, i, x, y).as_str();
        standart_draw_julia_set_threaded(j, &out_file.as_str(), x_fac * scale, y_fac * scale, depth);
        pb.inc();
    }
    pb.finish_println("done");
}

pub fn single_julia(jx: f64, jy: f64, scale: u32, out_file: &str, tries: u32) {
    let jul = Julia::new(jx, jy);
    main_julia(jul, -X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, out_file, tries)
}

pub fn raw_single_julia(jx: f64, jy: f64, scale: u32, tries: u32) -> Vec<u8> {
    let jul = Julia::new(jx, jy);
    raw_julia(jul, -X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, tries)
}

pub const X_DIF: f64 = 2.1333;
pub const Y_DIF: f64 = 1.2;

fn standart_draw_julia_set_threaded(julia: Julia, out_file: &str, x_range: u32, y_range: u32, depth: u32) {
    main_julia(julia, -X_DIF, X_DIF, -Y_DIF, Y_DIF, x_range, y_range, out_file, depth);
}

fn draw_cardioid(c: f64, slices: u32) {
    let xw = 1080;
    let yw = 1080;
    let mut img = RgbImage::new(xw, yw);
    
    for (i,(x, y)) in polar::rect_cardiod_range(c, 0.0, slices).enumerate() {
        println!("{} | {}", x, y);
        let rgb = {
            if i == 0 {
                Rgb([255,100,100])
            } else {
                Rgb([255,255,255])
            }
        };
        img.put_pixel(((xw / 2) as i32 + (x * (xw / 2)as f64) as i32) as u32, ((yw / 2) as i32 + (y * (yw / 2)as f64) as i32) as u32, rgb)
    }
    
    let out_file = "plots/cardiod.png";
    img.save(out_file).expect("oops");
}

pub fn main_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, out_file: &str, tries: u32) {
    let img = render_julia(julia, x_min, x_max, y_min, y_max, x_range, y_range, tries);
    img.save(out_file).expect("could not save image");
}

pub fn raw_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32) -> Vec<u8> {
    let img = render_julia(julia, x_min, x_max, y_min, y_max, x_range, y_range, tries);
    img.into_raw()
}

fn render_julia(julia: Julia, x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut handles = Vec::new();
    
    for (pix_x, cord_x) in convert_range(x_min, x_max, x_range) {
        let (tx, rx) = mpsc::channel();
        handles.push((rx, thread::spawn(move || {
            let mut line = Vec::new();
            
            for (pix_y, cord_y) in convert_range(y_min, y_max, y_range) {
                let i = julia.stable_cords(cord_x, cord_y, tries);
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

pub fn main_mandelbrot(scale: u32, out_file: &str, tries: u32) {
    let img = render_mandelbrot(-X_DIF, X_DIF, -Y_DIF, Y_DIF, 16 * scale , 9 * scale, tries);
    img.save(out_file).expect("could not save image");
}

pub fn fine_mandelbrot(x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, out_file: &str, tries: u32) {
    render_mandelbrot(x_min, x_max, y_min, y_max, x_range, y_range, tries).save(out_file).expect("could not save image")
}

fn render_mandelbrot(x_min: f64, x_max: f64, y_min:f64, y_max: f64, x_range: u32, y_range: u32, tries: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let mut img = RgbImage::new(x_range, y_range);
    let mut handles = Vec::new();
    
    for (pix_x, cord_x) in convert_range(x_min, x_max, x_range) {
        let (tx, rx) = mpsc::channel();
        handles.push((rx, thread::spawn(move || {
            let mut line = Vec::new();
            
            for (pix_y, cord_y) in convert_range(y_min, y_max, y_range) {
                let i = mandelbrot(cord_x, cord_y, tries);
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