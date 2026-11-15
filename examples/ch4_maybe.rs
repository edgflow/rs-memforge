//! MaybeUninit: room for a value that is not there yet.

use memforge_core::Record;
use std::mem::MaybeUninit;

fn main() {
    // Room for one Record: its size and its alignment, but no Record in it yet.
    let mut room = MaybeUninit::<Record>::uninit();
    println!("room: {} bytes, align {}", size_of_val(&room), align_of_val(&room));

    // write puts a whole value in. It is safe: it never reads what was there before.
    room.write(Record::new(b"user:1001", b"Behrouz"));

    // SAFETY: the write just above put a valid Record into `room`.
    let record: &Record = unsafe { room.assume_init_ref() };
    println!("borrowed: {record}");

    // SAFETY: the same valid Record. assume_init moves it out, and `room` is used up.
    let owned: Record = unsafe { room.assume_init() };
    println!("owned:    {} key bytes, {} value bytes", owned.key.len(), owned.value.len());
}
