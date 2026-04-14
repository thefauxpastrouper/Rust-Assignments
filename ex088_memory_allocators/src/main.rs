use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

const ARENA_SIZE: usize = 1024 * 1024;

struct ArenaAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    current: AtomicUsize
}

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for ArenaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        let size = layout.size();
        let align = layout.align();
        let arena = &mut *self.arena.get();
        let current = &mut self.current.load(Ordering::Relaxed);

        *current = (*current + align - 1) & !(align - 1);
        if *current + size > arena.len() {
            return std::ptr::null_mut();
        }

        let ptr = arena.as_mut_ptr().add(*current);
        *current += size;
        ptr
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

unsafe impl Sync for ArenaAllocator {}

#[global_allocator]
static GLOBAL: ArenaAllocator = ArenaAllocator {
    arena: UnsafeCell::new([0u8; ARENA_SIZE]),
    current: AtomicUsize::new(0)
};

fn main() {
    let count_before = ALLOC_COUNT.load(Ordering::Relaxed);
    let vec = vec![1,2,3,4,5];
    let count_after = ALLOC_COUNT.load(Ordering::Relaxed);

    println!("Vector: {:?}", vec);
    println!("Count: {}", count_after - count_before);
}
