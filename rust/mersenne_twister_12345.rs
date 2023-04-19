/*Name: Linyun Liu
Title: Explore Random Number Generator in Rust
 */
fn main() {
    // declare mutable rng object using Middle Square method
    let mut rng = MersenneTwister::new(9999, 0, 255);
    // print the next 10 random number
    for _ in 0..750000 {
        println!("{}", rng.next());
    }
}
// Define a struct to hold the state array and the current index
struct MersenneTwister{
    state: [u32; N],
    index: usize,
    start: u32,
    end: u32,
}
// ==================== Mersenne Twister ====================
// Constants used by the Mersenne Twister algorithm
const N: usize = 624; // Size of the state array
const M: usize = 397; // Parameter used in the twist function
const MATRIX_A: u32 = 0x9908B0DF; // Constant used in the twist function
const UPPER_MASK: u32 = 0x80000000; // Upper mask for the state values
const LOWER_MASK: u32 = 0x7FFFFFFF; // Lower mask for the state values

// Create a new Mersenne Twister instance with the given seed value
impl MersenneTwister {
    fn new(seed: u32, start: u32, end: u32) -> MersenneTwister {
        let mut mt = MersenneTwister {
            state: [0; N], // Initialize state array to all zeros
            index: N, // Current index into the state array
            start,
            end,
        };
        // Initialize the state array using the seed value
        mt.state[0] = seed;
        for i in 1..N {
            let x = mt.state[i - 1] ^ (mt.state[i - 1] >> 30);
            mt.state[i] = 1812433253u32.wrapping_mul(x).wrapping_add(i as u32);
        }
        mt
    }

    // Generate a new random number using the Mersenne Twister algorithm
    fn next(&mut self) -> u32 {
        // If we've reached the end of the state array, update the array
        if self.index >= N {
            self.twist();
        }
        // Extract a random number from the current state value
        let mut y = self.state[self.index];
        y ^= (y >> 11);
        y ^= (y << 7) & 0x9D2C_5680;
        y ^= (y << 15) & 0xEFC6_0000;
        y ^= (y >> 18);
        // Update the index and return the random number
        self.index += 1;
        // map the random number to the desired range
        let mut num = y % (self.end - self.start + 1) + self.start;
        // return the next random number
        num
    }

    // Update the state array using the twist function
    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK) + (self.state[(i + 1) % N] & LOWER_MASK);
            let mut xa = x >> 1;
            if x & 1 == 1 {
                xa ^= MATRIX_A;
            }
            self.state[i] = self.state[(i + M) % N] ^ xa;
        }
        self.index = 0;
    }
}

