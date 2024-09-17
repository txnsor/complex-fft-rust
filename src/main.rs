use complex_number::Complex;

pub mod complex_number;

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    println!("Hello, world!\n");

    let A: Complex<i16> = Complex {real: 10, imaginary: 20};
    let B: Complex<i16> = Complex {real: 40, imaginary: 30};
    let C: Complex<f64> = Complex {real: 10.1, imaginary: 1.111989};
    
    println!("A Angle: {} radians", A.angle());
    println!("B = {}", B.to_string());
    println!("A + B = {}", (A + B).to_string());
    println!("C Magnitude = {}", C.magnitude());
}
