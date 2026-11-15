//! MEMFORGE, measured: each benchmark runs the same work on both sides.

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use memforge::Database;
use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;
use std::alloc;
use std::hint::black_box;
use std::path::PathBuf;

mod baseline;
use baseline::Baseline;

/// A fresh, empty directory for one database.
fn fresh(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("memforge-bench").join(name);
    let _ = std::fs::remove_dir_all(&dir); // it may not exist yet
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The keys every engine benchmark uses: key:0 to key:999.
fn keys() -> Vec<Vec<u8>> {
    (0..1000).map(|i| format!("key:{i}").into_bytes()).collect()
}

const VALUE: &[u8] = b"a value of twenty-four b";

fn pages(c: &mut Criterion) {
    let mut g = c.benchmark_group("page");
    g.bench_function("vec![0u8; 4096]", |b| b.iter(|| black_box(vec![0u8; PAGE_SIZE])));
    g.bench_function("alloc_zeroed, aligned", |b| {
        b.iter(|| black_box(AlignedPage::allocate()))
    });
    let layout = AlignedPage::layout();
    g.bench_function("alloc, aligned, not zeroed", |b| {
        b.iter(|| {
            // SAFETY: the layout is not zero-sized; the page is freed with the same layout,
            // and never read.
            unsafe { alloc::dealloc(black_box(alloc::alloc(layout)), layout) }
        })
    });
    g.finish();
}

fn reads(c: &mut Criterion) {
    let page = AlignedPage::allocate();
    let mut g = c.benchmark_group("read 64 bytes");
    g.bench_function("slice", |b| b.iter(|| black_box(&page.as_bytes()[1234..1298])));
    g.bench_function("PageView", |b| b.iter(|| black_box(page.view().bytes(1234, 64))));
    g.finish();
}

fn engine(c: &mut Criterion) {
    let keys = keys();
    let mut g = c.benchmark_group("1000 records");
    g.sample_size(20);
    g.bench_function("MEMFORGE put + flush", |b| {
        b.iter_batched(
            || Database::open(fresh("put")).unwrap(),
            |mut db| {
                for k in &keys {
                    db.put(k, VALUE).unwrap();
                }
                db.flush().unwrap();
            },
            BatchSize::PerIteration,
        )
    });
    g.bench_function("baseline put + flush", |b| {
        b.iter_batched(
            || Baseline::open(&fresh("put-base").join("base.log")).unwrap(),
            |mut db| {
                for k in &keys {
                    db.put(k, VALUE).unwrap();
                }
                db.flush().unwrap();
            },
            BatchSize::PerIteration,
        )
    });

    let mut db = Database::open(fresh("get")).unwrap();
    let mut base = Baseline::open(&fresh("get-base").join("base.log")).unwrap();
    for k in &keys {
        db.put(k, VALUE).unwrap();
        base.put(k, VALUE).unwrap();
    }
    db.flush().unwrap();
    base.flush().unwrap();
    assert_eq!((db.len(), base.len()), (1000, 1000), "every key is on both sides");
    g.bench_function("MEMFORGE get", |b| {
        b.iter(|| keys.iter().for_each(|k| _ = black_box(db.get(k).unwrap())))
    });
    g.bench_function("baseline get", |b| {
        b.iter(|| keys.iter().for_each(|k| _ = black_box(base.get(k))))
    });
    g.finish();
}

criterion_group!(benches, pages, reads, engine);
criterion_main!(benches);
