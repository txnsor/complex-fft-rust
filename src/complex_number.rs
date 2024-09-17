/**
 * complex-numbers.rs
 * Basic generic complex numbers using num_traits.
 */
pub mod complex_number {
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]

    pub struct complex<T: Num> {
        real : T,
        imaginary: T
    }

    // since num_traits uses deprecated functionality, I redefined it

    // numeric operations
    pub trait NumOps<R = Self, O = Self>:  Add<R, O = O> +
    Sub<R, O = O> + Mul<R, O = O> + Div<R, O = O>
    + Rem<R, O = O> {}

    impl<T, R, O> NumOps<T, R, O> for T where
    T: Add<R, O = O> + Sub<R, O = O> + 
    Mul<R, O = O> + Div<R, O = O>
    + Rem<R, O = O> {}


    // number trait
    pub trait Num: PartialEq + Zero + One + NumOps {}

}