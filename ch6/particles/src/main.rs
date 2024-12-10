// std::alloc provides facilities for controlling memory allocation.
use std::alloc::{GlobalAlloc, Layout, System};
// std::time provides access to the system’s clock.
use std::time::Instant;
// rand provides random number generators and related functionality.
use rand::prelude::ThreadRng;
// graphics::math::Vec2d provides mathematical operations and conversion functionality for 2D vectors.
use graphics::math::Vec2d;

// #[global_allocator] marks the following value (ALLOCATOR) as satisfying the GlobalAlloc trait.
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

// Prints the time taken for each allocation to STDERR as the program runs.
// This provides a fairly accurate indication of the time taken for dynamic memory allocation.
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        // Defers the actual memory allocation to the system’s default memory allocator.
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        // https://doc.rust-lang.org/std/macro.eprintln.html
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

// Contains the data that is useful for the lifetime of the program.
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

fn main() {
}
