#[allow(non_camel_case_types)]

pub struct c16 {
    pub real: i8,
    pub complex: i8
}

fn main() {
    let complex = c16 {real : 10, complex: 20};
    println!("{} + {}i", complex.real, complex.complex);
}
