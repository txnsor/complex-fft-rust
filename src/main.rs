pub mod complex_number;

fn main() {
    println!("Hello, world!");
    let myNum: complex_number::Complex<i16> = Complex {real: 10, imaginary: 20};
}
