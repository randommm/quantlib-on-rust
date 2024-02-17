use ffi::run_integral;

#[cxx::bridge()]
mod ffi {
    unsafe extern "C++" {
        include!("quantlib-on-rust/include/some_example.h");

        fn run_integral() -> i32;
    }
}

fn main() {
    run_integral();
}
