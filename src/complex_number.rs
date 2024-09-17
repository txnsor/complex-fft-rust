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

// operations over Complex<T> are derived from T, assuming T has partialEq and NumOps

impl<T: Num<T>> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        return Complex {real: self.real + rhs.real, imaginary: self.imaginary + rhs.imaginary}
    }
}

// basic numeric traits

// numeric operations
pub trait NumOps<T>:  Add<T, Output = T> +
    Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>
    + Rem<T, Output = T> {}

// impl<T, R, Output> NumOps<R, Output> for T where
// T: Add<R, Output = Output> + Sub<R, Output = Output> + 
// Mul<R, Output = Output> + Div<R, Output = Output>
// + Rem<R, Output = Output> {}

// funny macro
// #[duplicate_item(name; [i8]; [i16]; [i32]; 
//     [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128]; [f32]; [f64];)]
// impl Num<name> for name {
//     fn eq(&self, other: &Self) -> bool {
//         self == 
//     }
// }


// number trait
pub trait Num<T>: PartialEq + NumOps<T> {}
