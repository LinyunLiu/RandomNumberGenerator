# Name: Linyun Liu
# Title: Explore Random Number Generator - Test

from PRNG import *

RNG1 = RNGMiddleSquare(255)
RNG2 = RNGLinearCon(1664525, 1013904223, pow(2, 32), 12234)
RNG3 = RNGMersenneTwister(12345)

count = 0
line = 0
while count < 1000:
    if line == 30:
        print()
        line = 0
    else:
        print(RNG3.random(0, 100), end=" ")
        line += 1
    count += 1
