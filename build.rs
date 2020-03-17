use std::env;

fn main() {
    env::set_var("LLVM_SYS_90_PREFIX", "/usr/local/opt/llvm");
    // println!("cargo:rustc-env=LLVM_SYS_90_PREFIX=/usr/local/opt/llvm");
}