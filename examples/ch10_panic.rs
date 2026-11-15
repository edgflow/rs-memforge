//! A panic at the boundary, called from Rust as C would call it.

use memforge_ffi::mf_demo_panic;

fn main() {
    let guarded = std::env::args().nth(1).as_deref() == Some("guarded");
    let code = mf_demo_panic(guarded as i32);
    println!("returned {code}");
}
