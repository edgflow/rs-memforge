//! The whole MEMFORGE API in one program: open, put, get, flush, close.

use memforge::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::open("./data")?;
    println!("opened {}", db.dir().display());

    db.put(b"user:1001", b"Behrouz")?;
    db.put(b"user:1002", b"Rust Developer")?;

    let value = db.get(b"user:1001")?.map(|v| String::from_utf8_lossy(&v).into_owned());
    println!("{value:?}");

    db.flush()?;
    db.close()?;
    Ok(())
}
