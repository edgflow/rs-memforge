//! MEMFORGE, measured: each benchmark runs the same work on both sides.

use criterion::{Criterion, criterion_group, criterion_main};
use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;
use std::alloc;
use std::hint::black_box;

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

criterion_group!(benches, pages);
criterion_main!(benches);
