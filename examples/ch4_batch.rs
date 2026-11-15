//! A batch of records with different sizes, written one at a time.

use memforge_core::Record;
use memforge_memory::initialization::Batch;

fn main() {
    let mut batch: Batch<Record, 4> = Batch::new();
    batch.push(Record::new(b"user:1001", b"Behrouz")).unwrap();
    batch.push(Record::new(b"user:1002", b"Rust Developer")).unwrap();
    batch.push(Record::new(b"log:7", &[b'x'; 300])).unwrap();
    println!("{} records, {} bytes each in the batch", batch.len(), size_of::<Record>());

    for index in 0..4 {
        match batch.get(index) {
            Some(r) => println!(
                "  {index}: key {} bytes, value {} bytes",
                r.key.len(),
                r.value.len()
            ),
            None => println!("  {index}: no value yet"),
        }
    }

    let last = batch.pop().unwrap();
    println!("popped a {}-byte value; {} left", last.value.len(), batch.len());
    // `last` and `batch` are dropped here: every record exactly once.
}
