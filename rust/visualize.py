# Name: Linyun Liu
# Title: Explore Random Number Generator in Rust

from PIL import Image

dimension = 500
img = Image.new('RGB', (dimension, dimension))

# read the file which contains random numbers from a specific PRNG
file = open("mersenne_twister_12345.txt", "r")

print("generating image...")
for i in range(dimension):
    for j in range(dimension):
        rgb = (int(file.readline()), int(file.readline()), int(file.readline()))
        img.putpixel((i, j), rgb)
img.save('image.jpeg')
