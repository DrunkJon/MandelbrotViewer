use std::ops::{Add, Sub, Mul, Div, Neg};

use bigdecimal::{BigDecimal, Zero, ToPrimitive};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    real: BigDecimal,
    imag: BigDecimal,
}

impl Complex {
    pub fn new(real: BigDecimal, imag: BigDecimal) -> Self {
        Self {
            real: real,
            imag: imag,
        }
    }

    pub fn null() -> Self {
        Self {
            real: BigDecimal::zero(),
            imag: BigDecimal::zero()
        }
    }

    pub fn con(&self) -> Self {
        Self {
            real: self.real.clone(),
            imag: - self.imag.clone(),
        }
    }

    // performance critical function
    pub fn dist_from_origin(&self) -> f64 {
        let real = self.real.to_f64().expect("could not convert BigDecimal to f64");
        let imag = self.real.to_f64().expect("could not convert BigDecimal to f64");
        let x = real.powi(2) + imag.powi(2);
        x.sqrt()
    }
    
    pub fn powi(self, exponent: u32) -> Self {
        if exponent == 0 {
            Complex::new(BigDecimal::from_str("1.0").unwrap(), BigDecimal::zero())
        } else {
            let mut z = self.clone();
            for _ in 0..exponent {
                z = z * self.clone();
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

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real.clone() * other.real.clone() - self.imag.clone() * other.imag.clone(),
            imag: self.imag.clone() * other.real.clone() + self.real.clone() * other.imag.clone(),
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.imag == BigDecimal::zero() {
            Self {
                real: self.real.clone() / other.real.clone(),
                imag: self.imag.clone() / other.real.clone(),
            }
        } else {
            let con = other.con();
            let num = self * con.clone();
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

/*
#[cfg(test)]
mod test {
    use super::Complex;
    use bigdecimal::{BigDecimal, Zero};
    use std::str::FromStr;

    #[test]
    fn complex_add_sub() {
        let a = Complex::new(BigDecimal::from_str("1.0").unwrap(), BigDecimal::zero());
        let b = Complex::new(BigDecimal::zero(), BigDecimal::from_str("1.0").unwrap());
        let c = Complex::new(BigDecimal::from_str("5.0").unwrap(), BigDecimal::from_str("3.0").unwrap());
        let d = Complex::new(BigDecimal::from_str("-2.0").unwrap(), BigDecimal::from_str("-6.0").unwrap());
    
        assert_eq!(a + a, Complex::new(BigDecimal::from_str("2.0").unwrap(), BigDecimal::zero()));
        assert_eq!(b + b, Complex::new(BigDecimal::zero(), BigDecimal::from_str("2.0").unwrap()));
        assert_eq!(a + b, Complex::new(BigDecimal::from_str("1.0").unwrap(), BigDecimal::from_str("1.0").unwrap()));
    
        assert_eq!(c - a, Complex::new(BigDecimal::from_str("4.0").unwrap(), BigDecimal::from_str("3.0").unwrap()));
        assert_eq!(c - b, Complex::new(BigDecimal::from_str("5.0").unwrap(), BigDecimal::from_str("2.0").unwrap()));
    
        assert_eq!(a + d, Complex::new(BigDecimal::from_str("-1.0").unwrap(), BigDecimal::from_str("-6.0").unwrap()));
        assert_eq!(b + d, Complex::new(BigDecimal::from_str("-2.0").unwrap(), BigDecimal::from_str("-5.0").unwrap()));
    
        assert_eq!(c + d, Complex::new(BigDecimal::from_str("3.0").unwrap(), BigDecimal::from_str("-3.0").unwrap()));
        assert_eq!(c - d, Complex::new(BigDecimal::from_str("7.0").unwrap(), BigDecimal::from_str("9.0").unwrap()));
    }
    
    #[test]
    fn complex_mul() {
        let a = Complex::new(BigDecimal::from_str("1.0").unwrap(),BigDecimal::zero());
        let b = Complex::new(BigDecimal::zero(),BigDecimal::from_str("1.0").unwrap());
        let c = Complex::new(BigDecimal::from_str("2.0").unwrap(),BigDecimal::from_str("3.0").unwrap());
        let d = Complex::new(BigDecimal::from_str("-5.0").unwrap(),BigDecimal::from_str("-4.0").unwrap());

        assert_eq!(a * b, Complex::new(BigDecimal::zero(), BigDecimal::from_str("1.0").unwrap()));
        
        assert_eq!(a * c, c);
        assert_eq!(b * c, Complex::new(BigDecimal::from_str("-3.0").unwrap(), BigDecimal::from_str("2.0").unwrap()));

        assert_eq!(c * d, Complex::new(BigDecimal::from_str("2.0").unwrap(), BigDecimal::from_str("-23.0").unwrap()));
    }

    #[test]
    fn complex_div() {
        let a = Complex::new(BigDecimal::from_str("1.0").unwrap(),BigDecimal::zero());
        let b = Complex::new(BigDecimal::zero(),BigDecimal::from_str("1.0").unwrap());
        let c = Complex::new(BigDecimal::from_str("2.0").unwrap(),BigDecimal::from_str("3.0").unwrap());
        let d = Complex::new(BigDecimal::from_str("-5.0").unwrap(),BigDecimal::from_str("-4.0").unwrap());

        assert_eq!(c / a, c);
        assert_eq!(d / a, d);

        assert_eq!(c / b, Complex::new(BigDecimal::from_str("3.0").unwrap(),BigDecimal::from_str("-2.0").unwrap()));
        assert_eq!(d / b, Complex::new(BigDecimal::from_str("-4.0").unwrap(),BigDecimal::from_str("5.0").unwrap()));
    }

    #[test]
    fn dist_from_origin() {
        let a = Complex::new(BigDecimal::from_str("1.0").unwrap(),BigDecimal::zero());
        let b = Complex::new(BigDecimal::zero(),BigDecimal::from_str("1.0").unwrap());
        let c = Complex::new(BigDecimal::from_str("2.0").unwrap(),BigDecimal::from_str("3.0").unwrap());
        let d = Complex::new(BigDecimal::from_str("-5.0").unwrap(),BigDecimal::from_str("-4.0").unwrap());

        assert_eq!(a.dist_from_origin(), 1f64);
        assert_eq!(b.dist_from_origin(), 1f64);
        assert_eq!(c.dist_from_origin(), 13f64.sqrt());
        assert_eq!(d.dist_from_origin(), 41f64.sqrt());
    }

    #[test]
    fn neg() {
        let num = Complex::new(BigDecimal::from_str("2.1").unwrap(), BigDecimal::from_str("-7.5").unwrap());
        assert_eq!(-num, Complex::new(BigDecimal::from_str("-2.1").unwrap(), BigDecimal::from_str("7.5").unwrap()));
    }
}
*/