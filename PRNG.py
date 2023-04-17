# Name: Linyun Liu
# Title: Explore Random Number Generator - Algorithms

# ==================== Middle Square Method ====================
class RNGMiddleSquare:

    def __init__(self, seed):
        self.seed = seed
        self.seed_length = len(str(seed))

    def random(self, start, end):
        # Calculate the number of digits in the range
        digits_count = len(str(end))
        # Square the seed
        seed_squared = self.seed * self.seed
        seed_squared_str = str(seed_squared)
        # Extract the middle digits
        while len(seed_squared_str) < self.seed_length * 2:
            seed_squared_str = "0" + seed_squared_str

        middle_digits = seed_squared_str[digits_count // 2:digits_count // 2 + digits_count]
        # Update the seed for the next random number
        self.seed = int(middle_digits)
        # Convert the middle digits to an integer within the range
        random_number = int(middle_digits) % (end - start + 1) + start
        # Return the random number
        return random_number

    def set_seed(self, seed):
        self.seed = seed


# ==================== Linear congruential generator ====================
class RNGLinearCon:

    def __init__(self, multiplier, increment, modulus, seed):
        self.multiplier = multiplier
        self.increment = increment
        self.modulus = modulus
        self.seed = seed

    def random(self, start, end):
        new_seed = (self.multiplier * self.seed + self.increment) % self.modulus
        self.seed = new_seed
        random_number = new_seed % (end - start + 1) + start
        return random_number

    def set_multiplier(self, multiplier):
        self.multiplier = multiplier

    def set_increment(self, increment):
        self.increment = increment

    def set_modulus(self, modulus):
        self.modulus = modulus

    def set_seed(self, seed):
        self.seed = seed


# ==================== Mersenne Twister ====================
class RNGMersenneTwister:
    def __init__(self, seed):
        # Initialize the MT19937 state with a seed value
        self.index = 624
        self.mt = [0] * 624
        self.mt[0] = seed
        for i in range(1, 624):
            self.mt[i] = 0xffffffff & (1812433253 * (self.mt[i - 1] ^ (self.mt[i - 1] >> 30)) + i)

    def random(self, start, end):
        # Extract a random integer from the MT19937 state
        if self.index >= 624:
            self.twist()

        y = self.mt[self.index]
        y = y ^ (y >> 11)
        y = y ^ ((y << 7) & 0x9d2c5680)
        y = y ^ ((y << 15) & 0xefc60000)
        y = y ^ (y >> 18)

        self.index += 1

        # Map the random integer to the desired range [start, end]
        return (y % (end - start + 1)) + start

    def twist(self):
        # Generate the next 624 random integers from the MT19937 state
        for i in range(624):
            y = (self.mt[i] & 0x80000000) + (self.mt[(i + 1) % 624] & 0x7fffffff)
            self.mt[i] = self.mt[(i + 397) % 624] ^ (y >> 1)
            if y % 2 != 0:
                self.mt[i] = self.mt[i] ^ 0x9908b0df
        self.index = 0
