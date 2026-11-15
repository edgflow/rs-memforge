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

fn reads(c: &mut Criterion) {
    let page = AlignedPage::allocate();
    let mut g = c.benchmark_group("read 64 bytes");
    g.bench_function("slice", |b| b.iter(|| black_box(&page.as_bytes()[1234..1298])));
    g.bench_function("PageView", |b| b.iter(|| black_box(page.view().bytes(1234, 64))));
    g.finish();
}

criterion_group!(benches, pages, reads);
criterion_main!(benches);
