/*Name: Linyun Liu
Student Number: 613705
CMPT 360 Spring 2023
Lab Assignment #7
Title: Explore Random Number Generator in Rust
 */
fn main() {
    // declare mutable rng object using Middle Square method
    let mut rng = MiddleSquare::new(999, 0, 255);
    // print the next 10 random number
    for _ in 0..750000 {
        println!("{}", rng.next());
    }
}

struct MiddleSquare {
    seed: u32,
    start: u32,
    end: u32,
}
// =================== Middle Square Method ====================
impl MiddleSquare {
    fn new(seed: u32, start: u32, end: u32) -> Self {
        Self { seed, start, end }
    }
    fn next(&mut self) -> u32 {
        // Calculate the number of digits in the range
        let digits_count = self.end.to_string().len();
        let seed_squared = self.seed * self.seed;
        // extract the middle digit
        let mut seed_squared_str = seed_squared.to_string();
        while seed_squared_str.len() < self.seed.to_string().len() * 2{
            seed_squared_str = "0".to_owned() + &seed_squared_str;
        }
        let mid = &seed_squared_str[(digits_count/2)..(digits_count/2+digits_count)];
        // update the seed
        let str = mid.to_string();
        let mut num: u32 = str.parse().unwrap();
        self.seed = num;
        // map the random number to the desired range
        num = num % (self.end - self.start + 1) + self.start;
        // return the seed as the next random number
        return num
    }
}