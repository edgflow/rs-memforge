//! Damage the data file on purpose, and see what recovery keeps.

use memforge::Database;
use std::fs::{self, OpenOptions};

const PAGE: u64 = 4096;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let case = std::env::args().nth(1).unwrap_or_default();
    let dir = "./data/damage";
    let _ = fs::remove_dir_all(dir);
    let mut db = Database::open(dir)?;
    for i in 0..3 {
        db.put(format!("key:{i}").as_bytes(), b"value")?;
        db.flush()?; // one page per flush: three pages
    }
    db.close()?;

    let path = format!("{dir}/memforge.db");
    match case.as_str() {
        "flip" => {
            let mut bytes = fs::read(&path)?;
            bytes[(PAGE + 30) as usize] ^= 0x01; // one bit, inside page 1's record
            fs::write(&path, bytes)?;
        }
        "truncate" => {
            let file = OpenOptions::new().write(true).open(&path)?;
            file.set_len(2 * PAGE + 1000)?; // page 2's write never finished
        }
        _ => {
            eprintln!("usage: ch9_damage flip | truncate");
            return Ok(());
        }
    }

    let db = Database::open(dir)?;
    println!("{:?}", db.recovery());
    for i in 0..3 {
        let key = format!("key:{i}");
        let found = db.get(key.as_bytes())?.is_some();
        println!("{key}: {}", if found { "found" } else { "missing" });
    }
    Ok(())
}
