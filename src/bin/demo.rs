//! The finished MEMFORGE in one run: open, put, get, close, a real page in hex, reopen.

use memforge::Database;
use std::fs;

fn text(value: Option<Vec<u8>>) -> Option<String> {
    value.map(|v| String::from_utf8_lossy(&v).into_owned())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "./data/demo";
    let _ = fs::remove_dir_all(dir); // start from an empty directory
    let mut db = Database::open(dir)?;
    println!("open    {dir}: {} pages on disk", db.recovery().pages);
    let records =
        [("user:1001", "Behrouz"), ("user:1002", "Rust Developer"), ("lang", "Rust")];
    for (key, value) in records {
        db.put(key.as_bytes(), value.as_bytes())?;
        println!("put     {key} = {value}");
    }
    println!("get     user:1002 -> {:?}", text(db.get(b"user:1002")?));
    db.close()?;
    println!("close   flushed, and synced to the disk");

    let bytes = fs::read(format!("{dir}/memforge.db"))?;
    println!("file    {} bytes: page 0, its first 64 in hex", bytes.len());
    for (row, chunk) in bytes[..64].chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        let shown: String = chunk
            .iter()
            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
            .collect();
        println!("  {:04x}  {}  {shown}", row * 16, hex.join(" "));
    }

    let db = Database::open(dir)?;
    println!("reopen  {:?}", db.recovery());
    println!("get     user:1001 -> {:?}", text(db.get(b"user:1001")?));
    Ok(())
}
