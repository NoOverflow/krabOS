cfg_match! {
    target_arch = "x86_64" => {
        pub mod x86_64;

        mod internal {
            pub use super::x86_64::*;
        }
    }
    _ = { compile_error!("KrabOS only supports x86_64 architecture.")}
}

pub fn init() {
    internal::init();
}
