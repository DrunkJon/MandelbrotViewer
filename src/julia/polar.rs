use std::f64::consts::PI;
use std::iter::Iterator;

pub type Iter<T> = Box<dyn Iterator<Item = T>>;

pub struct PolarSpace {
    anchor: (f64, f64)
}

impl PolarSpace {
    pub fn new(x: f64, y: f64) -> Self {
        PolarSpace{anchor:(x,y)}
    }

    pub fn polar_to_rectangular(self, r: f64, a: f64) -> (f64, f64) {
        let (x, y) = polar_to_rectangular(r, a);
        (x + self.anchor.0, y + self.anchor.1)
    }

    pub fn rectangular_to_polar(self, x: f64, y: f64) -> (f64, f64) {
        let x = x - self.anchor.0;
        let y = y - self.anchor.1;
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let a = (y / x).atan();
        (r,a)
    }
}

fn polar_range(offset: f64, slices: u32) -> Iter<f64> {
    partial_polar_range(offset, slices, 2.0 * PI)
}

fn partial_polar_range(offset: f64, slices: u32, max_angle: f64) -> Iter<f64> {
    Box::new((0..slices).map(move |i| {
        let a: f64 = (i as f64 / slices as f64) * max_angle;
        rotate(a, offset)
    }))
}

pub fn main_cardioid_path(slices: u32, offset: f64, max_angle: f64, derailment: f64) -> Iter<(f64, f64)> {
    let c = 0.5;
    Box::new(
        partial_cardioid_range(c, offset, slices, max_angle).map(move |(r,a)| {
            let space = PolarSpace::new(0.5 * c, 0.0);
            let r = r + derailment;
            let (x, y) = space.polar_to_rectangular(r, a);
            (x,y)
        })
    )
}

fn cardioid_range(c: f64, offset: f64, slices: u32) -> Iter<(f64, f64)> {
    partial_cardioid_range(c, offset, slices, 2.0 * PI)
}

fn partial_cardioid_range(c: f64, offset: f64, slices: u32, max_angle: f64) -> Iter<(f64, f64)> {
    Box::new(
        partial_polar_range(offset, slices, max_angle).map(move |a| {
            (cardioid(a, c), a)
        })
    )
}

pub fn rect_cardiod_range(c: f64, offset: f64, slices: u32) -> Iter<(f64, f64)> {
    partial_rect_cardioid_range(c, offset, slices, 2.0 * PI)
}

pub fn partial_rect_cardioid_range(c: f64, offset: f64, slices: u32, max_angle: f64) -> Iter<(f64, f64)> {
    Box::new(
        partial_cardioid_range(c, offset, slices, max_angle).map(move |(r,a)| {
            let (x, y) = polar_to_rectangular(r, a);
            let x = x + 0.5 * c;
            (x,y)
        })
    )
}

pub fn cardioid(a: f64, c: f64) -> f64 {
    c - c * a.cos()
}

pub fn rectangular_radial_path(angle: f64, r_min: f64, r_max: f64, slices: u32) -> Iter<(f64,f64)> {
    Box::new(
        radial_path(angle, r_min, r_max, slices).map(|(r, a)| {
            polar_to_rectangular(r, a)
        })
    )
}

fn radial_path(angle: f64, r_min: f64, r_max: f64, slices: u32) -> Iter<(f64, f64)> {
    let dif = r_max - r_min;
    Box::new(
        (0..slices+1).map(move |i| {
            (r_min + dif * i as f64 / (slices-1) as f64, angle)
        })
    )
}

fn rotate(a: f64, rotation: f64) -> f64 {
    let mut a = a + rotation;
    while a < 0.0 {
        a += 2.0 * PI;
    }
    while a >= 2.0 * PI {
        a -= 2.0 * PI;
    }
    a
}

fn angle_to_pi(angle: f64) -> f64 {
    angle / 180.0 * PI
}

fn polar_to_rectangular(r: f64, a: f64) -> (f64, f64) {
    let x = r * a.cos();
    let y = r * a.sin();
    (x, y)
}