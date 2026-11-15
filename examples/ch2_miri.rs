//! Two pointers that are not null, and still not valid. Run this only under Miri.

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
        "past-end" => {
            let record = [10u8, 20, 30, 40];
            // SAFETY: 4 is one past the end: a pointer we may compute, not read through.
            let end = unsafe { record.as_ptr().add(4) };
            // SAFETY: none. Reading past the end is the bug Miri has to find.
            println!("{}", unsafe { *end });
        }
        "dangling" => {
            let boxed = Box::new(42u32);
            let ptr: *const u32 = &*boxed;
            drop(boxed); // the heap block is freed here; ptr still holds its address
            println!("ptr = {ptr:p}, is_null: {}", ptr.is_null());
            // SAFETY: none. Reading freed memory is the bug Miri has to find.
            println!("{}", unsafe { *ptr });
        }
        _ => eprintln!("usage: ch2_miri past-end | dangling"),
    }
}
