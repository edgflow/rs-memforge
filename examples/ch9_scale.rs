//! How long recovery takes: 100,000 records written, then a new open rebuilds the index.

use memforge::Database;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "./data/scale";
    let _ = std::fs::remove_dir_all(dir);
    let mut db = Database::open(dir)?;
    for i in 0..100_000 {
        db.put(format!("user:{i}").as_bytes(), b"a value of twenty-four b")?;
    }
    db.close()?;

    let start = Instant::now();
    let db = Database::open(dir)?;
    let took = start.elapsed();
    let report = db.recovery();
    println!("{} pages, {} records, {} keys", report.pages, report.records, db.len());
    println!("recovered in {} ms", took.as_millis());
    Ok(())
}
