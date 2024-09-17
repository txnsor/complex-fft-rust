/**
 * complex-numbers.rs
 * Basic generic complex numbers using num_traits.
 */
#[allow(dead_code)]
use std::ops::*;
use duplicate::duplicate_item;

pub struct Complex<T: Num<T>> {
    real : T,
    imaginary: T
}
// generic functionality
impl<T: Num<T> + Copy> Complex<T> {
    fn magnitude(&self) -> T {
        return (self.real*self.real) + (self.imaginary*self.imaginary);
    }
}
// numeric type functionality
#[duplicate_item(name; [i8]; [i16]; [i32]; [u8]; [u16]; [u32]; [f32]; [f64];)]
impl Complex<name> {
    fn angle(&self) -> f64 {
        return f64::atan(f64::from(self.imaginary)/f64::from(self.real));
    }
}

// operations over Complex<T> are derived from T, assuming T has partialEq and NumOps

// addition
impl<T: Num<T>> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: self.real + rhs.real, imaginary: self.imaginary + rhs.imaginary}
    }
}

// subtraction
impl<T: Num<T>> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: self.real - rhs.real, imaginary: self.imaginary - rhs.imaginary}
    }
}

// basic numeric traits

// numeric operations
pub trait NumOps<T>:  Add<T, Output = T> +
    Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>
    + Rem<T, Output = T> {}
pub trait Num<T>: PartialEq + NumOps<T> {}

// impl<T, R, Output> NumOps<R, Output> for T where
// T: Add<R, Output = Output> + Sub<R, Output = Output> + 
// Mul<R, Output = Output> + Div<R, Output = Output>
// + Rem<R, Output = Output> {}

// macros to implement NumOps and Num on every primitive numeric type

#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
impl NumOps<name> for name {}

#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
impl Num<name> for name {}
