import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
import math

COLOUR_THRESHOLD = 250
SIZE_THRESHOLD = 1
LED_COUNT = 300

ignore_list = set([7])

def get_position(image_index):
    directions = [0, 90, 180, 270]
    centers = []

    for i, d in enumerate(directions):
        filename = f"Output/{d}/image{image_index}.png"
        try:
            im = Image.open(filename)
        except:
            continue

        img = im.load()

        (width, height) = im.size

        gray_scale = np.zeros((height, width))

        for x in range(width):
            for y in range(height):
                c = img[(x, y)]
                weight = c[0]/3 + c[1]/2 + c[2]/3
                gray_scale[y, x] = weight

        data = gray_scale >= COLOUR_THRESHOLD
        sum = np.sum(data)

        data = data / np.sum(np.sum(data))
        dx = np.sum(data, 0)
        dy = np.sum(data, 1)

        cx = np.sum(dx * np.arange(width))
        cy = np.sum(dy * np.arange(height))

        cx -= width / 2

        if sum >= SIZE_THRESHOLD:
            x = 0
            y = cy
            z = 0
            if i % 2 == 0:
                x = cx * (1 if i == 0 else -1)
            else:
                z = cx * (1 if i == 1 else -1)

            centers.append((x, y, z))
        else:
            centers.append((None, None, None))


    true_x = np.mean([p[0] for i, p in enumerate(centers) if i % 2 == 0 and not p[0] is None])
    true_y = height - np.mean([p[1] for p in centers if not p[1] is None])
    true_z = np.mean([p[2] for i, p in enumerate(centers) if i % 2 == 1 and not p[2] is None])

    if true_x is None or math.isnan(true_x):
        true_x = 0
    if true_y is None or math.isnan(true_y):
        true_y = 0
    if true_z is None or math.isnan(true_z):
        true_z = 0

    print(f"{image_index}: {centers} : {true_x}, {true_y}, {true_z}")
    print(f"Finished {image_index}")
    return (true_x, true_y, true_z)


data = [(x, get_position(x)) for x in range(LED_COUNT) if not x in ignore_list]

id = np.array([p[0] for p in data])
xs = np.array([p[1][0] for p in data])
ys = np.array([p[1][1] for p in data])
zs = np.array([p[1][2] for p in data])

y_offset = np.min(ys)
ys -= y_offset

with open("Output.pixels", "w") as file:
    for (id, x, y, z) in zip(id, xs, ys, zs):
        file.write(f"{id}: {x} {y} {z}\n")

fig = plt.figure()
ax = fig.add_subplot(projection='3d')

ax.scatter(xs, zs, ys)
ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

ax.set_xlim([-320, 320])
ax.set_ylim([320, -320])
ax.set_zlim([0, 480])
plt.show()
