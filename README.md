# A brief restatement of the problem
This project utilizes the high-level programming language Python to create a random number generator from scratch, without relying on any existing libraries. The aim is to implement the Pseudo Random Number Generator (PRNG) algorithms, and test out their effectiveness.

# Sample I/O (how the program is supposed to work)
There are three different PRNG algorithms defined in this project, each of them is defined in a class. The inputs are seeds that will passed as an argument for the constructors and range parameters for generating random numbers within that range.
**For Example:**
``` python
RNG1 = RNGMiddleSquare(255)
count = 0
line = 0
while count < 100:
    if line == 10:
        print()
        line = 0
    else:
        print(RNG1.random(0, 100), end=" ")
        line += 1
    count += 1
    ```
In this example, the RNGMiddleSquare require a seed input (255) as argument, and 0, 100 to specify the range of the random numbers.
**Sample Output:**
44 23 1 55 35 83 35 41 100 0 
26 85 72 37 30 54 26 41 6 28 
58 95 86 98 2 90 36 22 50 26 
14 87 95 28 58 95 86 98 2 90 
36 22 50 26 14 87 95 28 58 95 
86 98 2 90 36 22 50 26 14 87 
95 28 58 95 86 98 2 90 36 22 
50 26 14 87 95 28 58 95 86 98 
2 90 36 22 50 26 14 87 95 28 
58

# Documentation for a user to operate the program
*There are three algorithms/methods involved in this lab, Middle Square, Linear Congruential and Mersenne Twister.*
## THE ALGORITHMS
A. Middle Square Method:
1.	Initialize a seed value: Choose an integer number as the seed value for the random number generator. This value will be used to start the middle square method algorithm.
2.	Square the seed value: Multiply the seed value by itself to get a new number with (most likely) twice as many digits.
3.	Extract the middle digits: Take the middle digits of the squared value. If the squared value will have an even number of digits remained after the middle digits are extracted, take the middle digits in the middle. If it has an odd number of digits, then we add zero(s) before the digits before extracting.
4.	Update the seed: Use the middle digits as the next random number in the sequence. Store this value and repeat steps 2-4 to generate more random numbers.
 B. Linear Congruential
1.	Initialize a seed value: Choose a positive integer as the seed value for the random number generator. This value will be used to start the linear congruential method algorithm.
2.	Choose parameters: Choose three positive integers a, c, and m. These values will be used to define the linear congruential method algorithm. a great choice for these numbers are very important, luckily, many of these values have already been chosen and proven to be effective.
3.	Generate random numbers: in order to generate each random number in the sequence, we use the following formula: NewSeed = (a*Seed + c) mod m (a is called a multiplier, c is increment, and m is modulus)
4.	Repeat the process: Continue to repeat step 3 to generate as many random numbers as needed.
C. Mersenne Twister:
1.	Initialize state with a seed value: The first step is to initialize the state of the Mersenne Twister algorithm with a seed value. The seed value is provided as an input parameter to the Constructor of a RNGMersenneTwister class. The state is stored as an array of 624 32-bit integers, mt, and an index variable, index, which keeps track of the current position in the state array.
2.	Generate the next random number: To generate the next random number, the random method is called. This method first checks if the index has reached the end of the state array (624). If so, it calls the twist() method to generate the next 624 random integers and update the state array. Then, it extracts a single 32-bit integer, y, from the state array, and applies a series of bitwise operations to y to improve its statistical properties.
3.	Update the state: After extracting the random number, the random method updates the state Index to the next position In the state array, index += 1. 
4.	Generate the next 624 random integers: The twist method generates the next 624 random integers by performing a series of bitwise operations on the state array. For each element in the state array, it uses the previous 624 elements to calculate a new value, and then applies additional operations to this value to ensure good statistical properties.
5.	Repeat the process: To generate additional random numbers, the random method can be called repeatedly. Each time it is called, it generates a new random number and updates the state.

## RANGE
A good random number generator should allow user to specify a range of the outputs. In order to do that. The random method for all algorithms should maps the integer to the desired range [start, end] using the formula (y % (end - start + 1)) + start. This maps the integer to a value in the range [start, end].

## VERIFICATION
A good random number generator should also not repeat itself, in other words, it should generate numbers that will not result in patterns, not after a while at least. For example, a PRNG using middle square method will be most likely to generate a series of random numbers that will start to repeat itself as the middle digits may eventually equal to the initialized seed. We will need a way to detect such problem and then considering choosing a different seed or algorithm.

In this lab, we take advantages of python Pillow library to draw an image that represents the random numbers. To do that, we make the random number generator to generate numbers that are in the range 0 to 255 so that we can make that value to represent RGB values of a pixel. In theory, if the random number generator will repeat itself, the image should contain visible repeated texture. A good PRNG will produce an image that has no visible texture pattern, or at least, not after generating a large number of numbers
