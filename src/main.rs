use cxx::CxxVector;
use ffi::run_integral;

#[cxx::bridge()]
mod ffi {

    unsafe extern "C++" {
        include!("quantlib-on-rust/include/some_example.h");

        fn run_integral(to_integrate: fn(Vec<f64>) -> f64) -> i32;
    }
}

fn main() {
    run_integral(|x| {
        let mut sum = 1.;
        x.into_iter().for_each(|i| {
            sum *= (-i * i).exp() * i.cos();
        });
        sum
    });
}
