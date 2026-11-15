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
}
