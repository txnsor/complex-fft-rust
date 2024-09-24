/**
 * complex-numbers.rs
 * Basic generic complex numbers using num_traits.
 */

 // TODO: implement type casting between different complex number types.
#[allow(dead_code)]
use std::ops::*;
use duplicate::duplicate_item;

pub struct Complex<T: Num<T>> {
    pub(crate) real : T,
    pub(crate) imaginary: T
}
// generic functionality
impl<T: Num<T> + Copy> Complex<T> {
    pub fn magnitude(&self) -> T {
        return (self.real*self.real) + (self.imaginary*self.imaginary);
    }
}
// bounded, signed numeric type functionality
// if you want other functionality, code it yourself
#[duplicate_item(name; [i8]; [i16]; [i32]; [f32]; [f64];)]
impl Complex<name> {
    pub fn angle(&self) -> f64 {
        return f64::atan(f64::from(self.imaginary)/f64::from(self.real));
    }

    pub fn conjugate(&self) -> Complex<name> {
        return Complex {real: self.real, imaginary: self.imaginary.negate()}
    }
}

// numeric type code
#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
impl Complex<name> {
    pub fn to_string(&self) -> String {
        return format!("{} + {}*i", self.real, self.imaginary);
    }
}

// operations over Complex<T> are derived from T, assuming T has partialEq and NumOps

// addition
impl<T: Num<T> + Copy> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: self.real + rhs.real, imaginary: self.imaginary + rhs.imaginary}
    }
}

// subtraction
impl<T: Num<T> + Copy> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: self.real - rhs.real, imaginary: self.imaginary - rhs.imaginary}
    }
}

// TODO: compare arctan implementation to by parts implementation
// multiplication
impl<T: Num<T> + Copy> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: (self.real * rhs.real) - (self.imaginary * rhs.imaginary), imaginary: (self.imaginary * rhs.real) + (self.real * rhs.imaginary) }
    }
}

// divide (only for bounded numeric types)
#[duplicate_item(name; [i8]; [i16]; [i32]; [f32]; [f64];)]
impl Div<Complex<name>> for Complex<name> {
    type Output = Complex<name>;
    fn div(self, rhs: Complex<name>) -> Complex<name> {
        let pre = self*rhs.conjugate();
        let denom: name = rhs.magnitude();
        return Complex {real: pre.real/denom, imaginary: pre.imaginary/denom}
    }
}

// numeric operations
pub trait NumOps<T>:  Add<T, Output = T> +
    Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> {}
pub trait Neg<T> {fn negate(&self) -> T;}
pub trait Num<T>: PartialEq + NumOps<T> {}

// macros to implement NumOps and Num on every primitive numeric type

#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
impl NumOps<name> for name {}

#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
impl Num<name> for name {}

// macro on signed numbers
#[duplicate_item(name; [i8]; [i16]; [i32]; 
    [i64]; [i128]; [f32]; [f64];)]
    impl Neg<name> for name {
        fn negate(&self) -> name {
            return (-1 as name)*self;
        }
    }
