//! Raw pointers: an address with a type, and none of a reference's promises.

fn main() {
    let value = 42u32;

    // Making a raw pointer is safe: it is only an address.
    let ptr: *const u32 = &value;
    println!("ptr   = {ptr:p}");

    // Reading through it is not: nothing guarantees the address is still good.
    // SAFETY: ptr comes from `value` just above, and `value` is still alive.
    let read = unsafe { *ptr };
    assert_eq!(read, 42);
    println!("*ptr  = {read}");

    // A *mut pointer may also write.
    let mut count = 0u32;
    let count_ptr: *mut u32 = &mut count;
    // SAFETY: count_ptr comes from `count`, which is alive, and nothing else uses it here.
    unsafe { *count_ptr += 1 };
    println!("count = {count}");

    // A null pointer has address zero: it points at nothing. Making one is safe too.
    let null: *const u32 = std::ptr::null();
    println!("null  = {null:p}, is_null: {}", null.is_null());

    // Pointer arithmetic: add(n) moves n elements of the pointer's type, not n bytes.
    let record = [10u32, 20, 30, 40];
    let first = record.as_ptr();
    // SAFETY: index 2 is inside `record`, so the result stays inside the array.
    let third = unsafe { first.add(2) };
    println!("first = {first:p}, first.add(2) = {third:p}");
    // SAFETY: `third` points at record[2], an initialized u32, and `record` is alive.
    println!("*third = {}", unsafe { *third });
}
