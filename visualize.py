# Name: Linyun Liu
# Title: Explore Random Number Generator - Visualization

from PIL import Image
from PRNG import *

dimension = 500
seed = 12345
img = Image.new('RGB', (dimension, dimension))
RNG1 = RNGMiddleSquare(seed)
RNG2 = RNGLinearCon(75, 74, pow(2, 16)+1, seed)
# RNG2 = RNGLinearCon(25214903917, 11, pow(2, 48), seed)
RNG3 = RNGMersenneTwister(seed)

print("generating image...")
for i in range(dimension):
    for j in range(dimension):
        rgb = (RNG3.random(0, 255), RNG3.random(0, 255), RNG3.random(0, 255))
        img.putpixel((i, j), rgb)
img.save('image.jpeg')
