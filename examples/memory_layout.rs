//! Where MEMFORGE's bytes live: the stack, the heap, and their addresses.

fn main() {
    // A local variable lives on the stack, in main's own frame.
    let local = 42u32;
    // Box and Vec put their contents on the heap, and keep only a pointer here.
    let boxed = Box::new(7u32);
    let page = vec![0u8; 4096];

    println!("local       stack  {:p}", &local);
    println!("page (Vec)  stack  {:p}  {} bytes", &page, size_of_val(&page));
    println!("*boxed      heap   {:p}", &*boxed);
    println!("page data   heap   {:p}  {} bytes", page.as_ptr(), page.len());

    // Alignment: a value of a type must start at a multiple of the type's alignment.
    println!();
    println!("align_of  u8 {}  u16 {}", align_of::<u8>(), align_of::<u16>());
    println!("align_of u32 {}  u64 {}", align_of::<u32>(), align_of::<u64>());
    let words = [1u64, 2, 3];
    println!("three u64s at {:p} {:p} {:p}", &words[0], &words[1], &words[2]);
}
