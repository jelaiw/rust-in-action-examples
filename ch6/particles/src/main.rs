// std::alloc provides facilities for controlling memory allocation.
use std::alloc::{GlobalAlloc, Layout, System};
// std::time provides access to the system’s clock.
use std::time::Instant;
// rand provides random number generators and related functionality.
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
// graphics::math::Vec2d provides mathematical operations and conversion functionality for 2D vectors.
use graphics::math::{Vec2d, add, mul_scalar};

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

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            // Uses Box<Particle> rather than Particle to incur an extra memory allocation when every particle is created.
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            // Creates a Particle as a local variable on the stack.
            let particle = Particle::new(&self);
            // Takes ownership of particle, moving its data to the heap, and creates a reference to that data on the stack.
            let boxed_particle = Box::new(particle);
            // Pushes the reference into self.shapes.
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            // particle_iter is split into its own variable to more easily fit on the page.
            let particle_iter = self.particles.iter().enumerate();

            // For n iterations, removes the first particle that’s invisible.
            // If there are no invisible particles, then removes the oldest.
            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            }
            else {
                self.particles.remove(0);
            };
        }
    }

    fn update(&mut self) {
        // Returns a random integer between –3 and 3, inclusive.
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        }
        else {
            self.remove_shapes(n);
        }
    
        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        // Starts at a random position along the bottom of the window.
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        // Rises vertically over time.
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        // Increases the speed of the rise over time.
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            // into() converts the arrays of type [f64; 2] into Vec2d.
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            // Inserts a fully saturated white that has a tiny amount of transparency.
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        // Moves the particle to its next position.
        self.position = add(self.position, self.velocity);
        // Slows down the particle’s rate of increase as it travels across the screen.
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        // Makes the particle more transparent over time.
        self.color[3] *= 0.995;
    }
}

fn main() {
}
