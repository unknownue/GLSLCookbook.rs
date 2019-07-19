
use std::time::Instant;

const FPS_SAMPLE_COUNT: usize = 5;
const FPS_SAMPLE_COUNT_FLOAT: f32 = FPS_SAMPLE_COUNT as f32;


pub struct Timer {

    counter: Instant,

    samples: [u32; FPS_SAMPLE_COUNT],
    current_frame: usize,
    delta_frame: u32, // unit microseconds
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            counter: Instant::now(),
            samples: [0; FPS_SAMPLE_COUNT],
            current_frame: 0,
            delta_frame: 0,
        }
    }

    /// Call this function in each frame during game loop to update its inner status.
    pub fn tick_frame(&mut self) {
        let time_elapsed = self.counter.elapsed();
        self.counter = Instant::now();

        self.delta_frame = time_elapsed.subsec_micros();
        self.samples[self.current_frame] = self.delta_frame;
        self.current_frame = (self.current_frame + 1) % FPS_SAMPLE_COUNT;
    }

    /// Calculate the current FPS.
    #[allow(dead_code)]
    pub fn fps(&self) -> f32 {

        let sum: u32 = self.samples.iter().sum();
        1000_000.0_f32 / (sum as f32 / FPS_SAMPLE_COUNT_FLOAT)
    }

    /// Return current delta time in seconds.
    /// this function ignore its second part, since the second is mostly zero.
    #[inline]
    pub fn delta_time(&self) -> f32 {
        self.delta_frame as f32 / 1000_000.0_f32 // time in second
    }
}
