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
```
44 23 1 55 35 83 35 41 100 0 
26 85 72 37 30 54 26 41 6 28 <img width="500" alt="image" src="https://user-images.githubusercontent.com/90886678/232553617-9a776234-4d3f-4773-b6bb-a190c4c55f1b.png">

58 95 86 98 2 90 36 22 50 26 
14 87 95 28 58 95 86 98 2 90 
36 22 50 26 14 87 95 28 58 95 
86 98 2 90 36 22 50 26 14 87 
95 28 58 95 86 98 2 90 36 22 
50 26 14 87 95 28 58 95 86 98 
2 90 36 22 50 26 14 87 95 28 
58
```

# Documentation for a user to operate the program
*There are three algorithms/methods involved in this lab, Middle Square, Linear Congruential and Mersenne Twister.*
## THE ALGORITHMS
A. Middle Square Method:
* Initialize a seed value: Choose an integer number as the seed value for the random number generator. This value will be used to start the middle square method algorithm.
* Square the seed value: Multiply the seed value by itself to get a new number with (most likely) twice as many digits.
* Extract the middle digits: Take the middle digits of the squared value. If the squared value will have an even number of digits remained after the middle digits are extracted, take the middle digits in the middle. If it has an odd number of digits, then we add zero(s) before the digits before extracting.
4.Update the seed: Use the middle digits as the next random number in the sequence. Store this value and repeat steps 2-4 to generate more random numbers.

B. Linear Congruential
* Initialize a seed value: Choose a positive integer as the seed value for the random number generator. This value will be used to start the linear congruential method algorithm.
* Choose parameters: Choose three positive integers a, c, and m. These values will be used to define the linear congruential method algorithm. a great choice for these numbers are very important, luckily, many of these values have already been chosen and proven to be effective.
* Generate random numbers: in order to generate each random number in the sequence, we use the following formula: NewSeed = (a*Seed + c) mod m (a is called a multiplier, c is increment, and m is modulus)
* Repeat the process: Continue to repeat step 3 to generate as many random numbers as needed.

C.Mersenne Twister:
* Initialize state with a seed value: The first step is to initialize the state of the Mersenne Twister algorithm with a seed value. The seed value is provided as an input parameter to the Constructor of a RNGMersenneTwister class. The state is stored as an array of 624 32-bit integers, mt, and an index variable, index, which keeps track of the current position in the state array.
* Generate the next random number: To generate the next random number, the random method is called. This method first checks if the index has reached the end of the state array (624). If so, it calls the twist() method to generate the next 624 random integers and update the state array. Then, it extracts a single 32-bit integer, y, from the state array, and applies a series of bitwise operations to y to improve its statistical properties.
* Update the state: After extracting the random number, the random method updates the state Index to the next position In the state array, index += 1. 
* Generate the next 624 random integers: The twist method generates the next 624 random integers by performing a series of bitwise operations on the state array. For each element in the state array, it uses the previous 624 elements to calculate a new value, and then applies additional operations to this value to ensure good statistical properties.
* Repeat the process: To generate additional random numbers, the random method can be called repeatedly. Each time it is called, it generates a new random number and updates the state.

## RANGE
A good random number generator should allow user to specify a range of the outputs. In order to do that. The random method for all algorithms should maps the integer to the desired range `[start, end]` using the formula `(y % (end - start + 1)) + start`. This maps the integer to a value in the range `[start, end]`.

## VERIFICATION
A good random number generator should also not repeat itself, in other words, it should generate numbers that will not result in patterns, not after a while at least. For example, a PRNG using middle square method will be most likely to generate a series of random numbers that will start to repeat itself as the middle digits may eventually equal to the initialized seed. We will need a way to detect such problem and then considering choosing a different seed or algorithm.

In this lab, we take advantages of python Pillow library to draw an image that represents the random numbers. To do that, we make the random number generator to generate numbers that are in the range 0 to 255 so that we can make that value to represent RGB values of a pixel. In theory, if the random number generator will repeat itself, the image should contain visible repeated texture. A good PRNG will produce an image that has no visible texture pattern, or at least, not after generating a large number of numbers

# Results 
*The printed image, texture may not be clear*
Each pixel stands for 3 consecutive random generated numbers; a 400 by 400 images hence contains 480,000 consecutive random generated numbers. 
480,000 of them should be sufficient to test the PRNG.
The images below are magnified, zoomed in so that the pixels are clear on a printed paper
## A.Using Middle Square Method:
**Image 1** (very zoomed in): the PRNG generates few random numbers and then it start to generate the same values using seed value 999
![image 1-2.png](https://github.com/LinyunLiu/RandomNumberGenerator/blob/main/image_examples/image%201-2.png?raw=true)   

**Image 2** (enlarged): the PRNG generates a ton of random numbers and then it start to repeat the same sequence using seed value 255
![image 1-1.png](https://github.com/LinyunLiu/RandomNumberGenerator/blob/main/image_examples/image%201-1.png?raw=true)
*After testing with many other seeds, middle square method appears to be quite inefficient, it is unstable and unreliable, 
the randomness is largely based on finding the good seed, and it seems there are no seed that will make sure the numbers don’t repeat*    


## B.Using Linear Congruential PRNG
**Image 3** (very roomed in): the PRNG generates tons of random numbers, and it appears that there is no significant repetition
`Seed: 12345`
`Multiplier: 75`
`Increment:74`
`Modulus: pow(2, 16)`
Used by ZX81, a home computer in 1981
![image 2-1.png](https://github.com/LinyunLiu/RandomNumberGenerator/blob/main/image_examples/image%202-1.png?raw=true)    

**Image 4** (zoomed in a little bit): the PRNG generates a lot of random numbers, and then it starts to repeat itself, 
but it lasts longer than middle square method
`Seed: 12345`
`Multiplier: 22695477`
`Increment: 1`
`Modulus: pow(2, 32)`
Used by Borland Software Corporation, a computer technology company
![image 2-2.png](https://github.com/LinyunLiu/RandomNumberGenerator/blob/main/image_examples/image%202-2.png?raw=true)
*Linear Congruential is much more reliable than using Middle Square method, however, it still not as good as we think. 
If we use the same seed, changing the multiplier, increment and modulus can significantly change the performance of PRNG as shown above. 
Some of the combination of these parameters seems always produce patterned random numbers no matter what seed we choose.*    


## C.Using Mersenne Twister:
It is said that the Mersenne Twister PRNG is one of the most reliable random number generators  
**Image 5** (very roomed in): the PRNG generates  tons of random numbers, and it appears that there is no significant repetition
`Seed: 12345`
![image 3-1.png](https://github.com/LinyunLiu/RandomNumberGenerator/blob/main/image_examples/image%203-1.png?raw=true)
*After testing with many different seeds, it seems that the Mersenne Twister algorithm is unbreakable, however, if the images is 5000 x 5000, 
which means 75,000,000 numbers are generated, the pattern seems to emerge, but it is barely visible, it is hard to make a judgement.*   


# Conclusion

The Middle Square Method is a very simple and straightforward algorithm that involves squaring a number and taking the middle digits as the 
next random number. While it is easy to implement, the quality of the generated numbers is not very high, and the algorithm suffers from weaknesses, 
such as it depends on very unique seed. In particular, it can quickly fall into repeating patterns and is not suitable for cryptographic applications.  

Linear Congruential Generator (LCG) is a popular RNG that is widely used due to its simplicity and efficiency. It involves multiplying the 
previous random number by a constant, adding another constant, and taking the result modulo a third constant to obtain the next random number. 
LCGs are fast and also easy to implement, but they can suffer from some weaknesses as well, such as poor statistical properties and predictability. 
They are not very suitable for cryptographic applications but are often used in simulations (such as in games) and other non-cryptographic contexts.  

Mersenne Twister is a high-quality RNG that is widely used in both cryptographic and non-cryptographic applications. 
It uses a large state vector (624 words) and a complex algorithm to generate high-quality random numbers. It can generate a vast 
number of unique sequences of random numbers before repeating itself. It has excellent statistical properties, and it is not slow at all.  

In conclusion, while the Middle Square Method and Linear Congruential Generator are both simple and are able to produce plenty 
random numbers (Linear Congruential Generator is much more efficient however), their predictability make them unsuitable for many applications. 
The Mersenne Twister, on the other hand, is a high-quality RNG with excellent statistical properties and a very long period, 
making it suitable for a wide range of applications, it is reliable and efficient enough.  

