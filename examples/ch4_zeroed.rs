//! A Vec made of zero bytes: the compiler warns, and Miri shows why. Run it under Miri.

fn main() {
    // SAFETY: none. All-zero bytes are not a valid Vec: this line is the bug.
    let v: Vec<u8> = unsafe { std::mem::zeroed() };
    println!("{}", v.len());
}
