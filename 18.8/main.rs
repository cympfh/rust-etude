use std::fmt;

#[link(name="m")]
extern {
    fn csqrtf(z: Complex) -> Complex;
}

fn sqrt(z: Complex) -> Complex {
    unsafe { csqrtf(z) }
}

fn main() {
    {
        let z = Complex(1., 0.);
        let y = sqrt(z);
        println!("sqrt({:?})= {:?}", z, y);
    }
    {
        let z = Complex(-1., 0.);
        let y = sqrt(z);
        println!("sqrt({:?})= {:?}", z, y);
    }
}

#[repr(C)]
#[derive(Clone,Copy)]
struct Complex(f32, f32);

impl fmt::Debug for Complex {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let Complex(re, im) = *self;
        write!(f, "{}i{}", re, im)
    }
}
