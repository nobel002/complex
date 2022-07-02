use std::ops::{Add, Div, Mul, Sub};

pub struct Complex {
    pub re: f64,
    pub im: f64,
}

pub trait ToCarthesian {
    fn to_carthesian(&self) -> Complex;
}
impl ToCarthesian for Complex {
    fn to_carthesian(&self) -> Complex {
        Complex {
            re: self.re * self.im.cos(),
            im: self.re * self.im.sin(),
        }
    }
}

pub trait ToPolar {
    fn to_polar(&self) -> Complex;
}
impl ToPolar for Complex {
    fn to_polar(&self) -> Complex {
        Complex {
            re: self.re.hypot(self.im),
            im: self.im.atan2(self.re),
        }
    }
}
// A and B are complex numbers.
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        // Conversion to carthesian coordinates && addition of the two points.
        let temporary = Complex {
            re: (self.re * self.im.cos()) + (other.re * other.im.cos()),
            im: (self.re * self.im.sin()) + (other.re * other.im.sin()),
        };
        temporary.to_polar()
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        // Conversion to carthesian coordinates
        let temporary = Complex {
            re: self.re * self.im.cos() - other.re * other.im.cos(),
            im: self.re * self.im.sin() - other.re * other.im.sin(),
        };
        temporary.to_polar()
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re,
            im: self.im + other.im,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {
            re: self.re / other.re,
            im: self.im - other.im,
        }
    }
}
// A is a Complex, B is real.
impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, other: f64) -> Complex {
        Complex {
            re: self.re * other,
            im: self.im,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, other: f64) -> Complex {
        Complex {
            re: self.re/other,
            im: self.im,
        }
    }
}
// A is a real, B is Complex.
impl Div<Complex> for f64 {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {
            re: self/other.re,
            im: - other.im,
        }
    }
}

impl Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self * other.re,
            im: other.im,
        }
    }
}

pub trait Sin {
    fn sin(&self) -> Complex;
}
impl Sin for Complex {
    fn sin(&self) -> Complex {
        let temp = Complex {
            re: self.to_carthesian().re.sin() * self.to_carthesian().im.cosh(),
            im: self.to_carthesian().re.cos() * self.to_carthesian().im.sinh(),
        };
        temp.to_polar()
    }
}

pub trait Cos {
    fn cos(&self) -> Complex;
}
impl Cos for Complex {
    fn cos(&self) -> Complex {
        let temp = Complex {
            re: self.to_carthesian().re.cos() * self.to_carthesian().im.cosh(),
            im: -self.to_carthesian().re.sin() * self.to_carthesian().im.sinh(),
        };
        temp.to_polar()
    }
}

pub trait Tan {
    fn tan(&self) -> Complex;
}
impl Tan for Complex {
    fn tan(&self) -> Complex {
        self.sin() / self.cos()
    }
}

pub trait Cot {
    fn cot(&self) -> Complex;
}
impl Cot for Complex {
    fn cot(&self) -> Complex {
        self.cos() / self.sin()
    }
}

pub trait Sec {
    fn sec(&self) -> Complex;
}
impl Sec for Complex {
    fn sec(&self) -> Complex {
        1.0 / self.cos()
    }
}

pub trait Csc {
    fn csc(&self) -> Complex;
}
impl Csc for Complex {
    fn csc(&self) -> Complex {
        1.0 / self.sin()
    }
}
