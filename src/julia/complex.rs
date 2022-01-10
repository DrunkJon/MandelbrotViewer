use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Neg};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self {
            real: real,
            imag: imag,
        }
    }

    pub fn null() -> Self {
        Self {
            real: 0.0,
            imag: 0.0
        }
    }

    pub fn con(&self) -> Self {
        Self {
            real: self.real,
            imag: - self.imag,
        }
    }

    pub fn dist_from_origin(&self) -> f64 {
        let x = self.real.powi(2) + self.imag.powi(2);
        x.sqrt()
    }
    
    pub fn powi(self, exponent: u32) -> Self {
        if exponent == 0 {
            Complex::new(1.0, 0.0)
        } else {
            let mut z = self;
            for _ in 0..exponent {
                z = z * self;
            }
            z
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real, 
            imag: self.imag + other.imag,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        };
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        };
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real.mul_add(other.real, - self.imag * other.imag),
            imag: self.imag.mul_add(other.real, self.real * other.imag),
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.imag == 0.0 {
            Self {
                real: self.real / other.real,
                imag: self.imag / other.real,
            }
        } else {
            let con = other.con();
            let num = self * con;
            let denom = other * con;
            num / denom
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            real: - self.real,
            imag: - self.imag,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Complex;

    #[test]
    fn complex_add_sub() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let c = Complex::new(5.0, 3.0);
        let d = Complex::new(-2.0, -6.0);
    
        assert_eq!(a + a, Complex::new(2.0, 0.0));
        assert_eq!(b + b, Complex::new(0.0, 2.0));
        assert_eq!(a + b, Complex::new(1.0, 1.0));
    
        assert_eq!(c - a, Complex::new(4.0, 3.0));
        assert_eq!(c - b, Complex::new(5.0, 2.0));
    
        assert_eq!(a + d, Complex::new(-1.0, -6.0));
        assert_eq!(b + d, Complex::new(-2.0, -5.0));
    
        assert_eq!(c + d, Complex::new(3.0, -3.0));
        assert_eq!(c - d, Complex::new(7.0, 9.0));
    }

    #[test]
    fn complex_add_sub_assign() {
        let mut a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let mut c = Complex::new(2.0, -3.0);
        let d = Complex::new(-2.0, -6.0);

        a += b;
        assert_eq!(a, Complex::new(1.0, 1.0));

        c -= d;
        assert_eq!(c, Complex::new(4.0, 3.0));
    }
    
    #[test]
    fn complex_mul() {
        let a = Complex::new(1.0,0.0);
        let b = Complex::new(0.0,1.0);
        let c = Complex::new(2.0,3.0);
        let d = Complex::new(-5.0,-4.0);

        assert_eq!(a * b, Complex::new(0.0, 1.0));
        
        assert_eq!(a * c, c);
        assert_eq!(b * c, Complex::new(-3.0, 2.0));

        assert_eq!(c * d, Complex::new(2.0, -23.0));
    }

    #[test]
    fn complex_div() {
        let a = Complex::new(1.0,0.0);
        let b = Complex::new(0.0,1.0);
        let c = Complex::new(2.0,3.0);
        let d = Complex::new(-5.0,-4.0);

        assert_eq!(c / a, c);
        assert_eq!(d / a, d);

        assert_eq!(c / b, Complex::new(3.0,-2.0));
        assert_eq!(d / b, Complex::new(-4.0,5.0));

        assert_eq!(c / d, Complex::new(-22.0 / 41.0, -7.0 / 41.0));
    }

    #[test]
    fn dist_from_origin() {
        let a = Complex::new(1.0,0.0);
        let b = Complex::new(0.0,1.0);
        let c = Complex::new(2.0,3.0);
        let d = Complex::new(-5.0,-4.0);

        assert_eq!(a.dist_from_origin(), 1f64);
        assert_eq!(b.dist_from_origin(), 1f64);
        assert_eq!(c.dist_from_origin(), 13f64.sqrt());
        assert_eq!(d.dist_from_origin(), 41f64.sqrt());
    }

    #[test]
    fn neg() {
        let num = Complex::new(2.1, -7.5);
        assert_eq!(-num, Complex::new(-2.1, 7.5));
    }
}