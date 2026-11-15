//! Values that survive a restart: written, closed, and read back by a new Database.

use memforge::Database;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "./data/reopen";
    let _ = fs::remove_dir_all(dir); // start from an empty directory
    let mut db = Database::open(dir)?;
    db.put(b"user:1001", b"Behrouz")?;
    db.put(b"user:1002", b"Rust Developer")?;
    db.put(b"user:1001", b"Behrouz R.")?; // a newer value for the same key
    db.put(b"flag:dark-mode", b"on")?;
    db.delete(b"flag:dark-mode")?;
    db.close()?;

    let db = Database::open(dir)?;
    println!("{:?}", db.recovery());
    for key in ["user:1001", "user:1002", "flag:dark-mode"] {
        let value = db.get(key.as_bytes())?;
        let text = value.map(|v| String::from_utf8_lossy(&v).into_owned());
        println!("{key} = {text:?}");
    }
    Ok(())
}
