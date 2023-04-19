/*Name: Linyun Liu
Student Number: 613705
CMPT 360 Spring 2023
Lab Assignment #7
Title: Explore Random Number Generator in Rust
 */
fn main() {
    // declare mutable rng object using Linear Congruential method
    let m = 2u128.pow(32);
    let mut rng = LinearCon::new(12345, 0, 255, 22695477, 1, m);
    /*
    Multiplier: 22695477
    Increment: 1
    Modulus: pow(2, 32)
    Used by Borland Software Corporation, a computer technology company
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