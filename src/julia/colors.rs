use std::f64::consts::PI;
use image::{Rgb};

// h: hue as a rotation on the color wheel [0, 2*PI]
// s: saturation [0,1]
// v; value (brightness) [0,1]
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let h = {
        if h < 2.0 * PI {
            h
        } else {
            h - 2.0 * PI
        }
    };
    let h_frac = h / (PI / 3.0);
    let f = h_frac - h_frac.floor();
    let p = v * (1.0-s);
    let q = v * (1.0 - s * f);
    let t = v* (1.0 - s * (1.0 - f));
    let (rf, gf,bf) = match h_frac.floor() {
        x if x == 0.0 => (v, t, p),
        x if x == 1.0 => (q, v, p),
        x if x == 2.0 => (p, v, t),
        x if x == 3.0 => (p, q, v),
        x if x == 4.0 => (t, p, v),
        x if x == 5.0 => (v, p, q),
        x if x == 6.0 => (v, t, p),
        _ => panic!("got illegal h_frac value in hsv_to_rgb: {}", h_frac)
    };
    let (r,g,b) = ((rf * 256.0).floor() as u8, (gf * 256.0).floor() as u8, (bf * 256.0).floor() as u8,);
    return (r,g,b)
}

// color function
pub fn ratio_to_color(ratio: f64) -> Rgb<u8> {
    let offset = 1.8 / 3.0;
    let h = (ratio + offset) * 2.0 * PI;
    let (r,g,b) = hsv_to_rgb(h, 0.6, 1.0);
    Rgb([r, g, b])
}

const STD_DEPTH: u32 = 100;

pub fn color_builder(i: u32) -> Rgb<u8> {
    let ratio = (i % STD_DEPTH) as f64 / STD_DEPTH as f64;
    ratio_to_color(ratio)
}