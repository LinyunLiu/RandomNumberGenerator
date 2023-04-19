/*Name: Linyun Liu
Title: Explore Random Number Generator in Rust
 */
fn main() {
    // declare mutable rng object using Linear Congruential method
    let m = 2u32.pow(16);
    let mut rng = LinearCon::new(12345, 0, 255, 75, 74, m+1);
    /*
    Multiplier: 75
    Increment:74
    Modulus: pow(2, 16)+1
    Used by ZX81, a home computer in 1981
    */
    // print the next 500x500x3 random numbers
    for _ in 0..750000 {
        println!("{}", rng.next());
    }
}
struct LinearCon{
    seed: u32,
    start: u32,
    end: u32,
    multiplier: u32,
    increment: u32,
    modulus: u128, // the multiplier can be huge, 32bits is not enough

}
// ==================== Linear congruential generator ====================
impl LinearCon{
    fn new(seed: u32, start: u32, end: u32, multiplier: u32, increment: u32, modulus: u128) -> Self {
        Self { seed, start, end, multiplier, increment, modulus }
    }
    fn next(&mut self) -> u32{
        // compute the next seed which is also the random number
        let new_seed = (self.multiplier as u128 * self.seed as u128 + self.increment as u128) % self.modulus;
        // update the seed
        self.seed = new_seed as u32;
        // map the random number to the desired range
        let num = new_seed as u32 % (self.end - self.start + 1) + self.start;
        // return the seed as the next random number
        return num as u32;
    }
}
