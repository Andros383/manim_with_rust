use std::ops::{Add, Mul, MulAssign};
//FIXME Talán clone és copy nélkül?
// A baj akkor van amikor a `let c:Complex;for i in 0..100{c=c*c+start_c}`
// Lusta 
#[derive(Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub complex: f64,
}
impl Complex {
    pub fn abs(&self) -> f64 {
        f64::sqrt(self.real.powi(2) + self.complex.powi(2))
    }
}
impl Mul<Self> for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.real;
        let b = self.complex;
        let c = rhs.real;
        let d = rhs.complex;
        return Complex {
            real: a * c - b * d,
            complex: a * d + b * c,
        };
    }
}
//TODO lehetne ezt gyorsítani? Szerintem ez csak ki lesz optimalizálva a fenébe
impl MulAssign<Self> for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let a = self.real;
        let b = self.complex;
        let c = rhs.real;
        let d = rhs.complex;

        self.real = a * c - b * d;
        self.complex = a * d + b * c;
    }
}
impl Add<Self> for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            complex: self.complex + rhs.complex,
        }
    }
}
