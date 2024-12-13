import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
import math

COLOUR_THRESHOLD = 250
SIZE_THRESHOLD = 1
LED_COUNT = 300

# ignore_list = set([7])

# display_list = set([15, 60, 62])

# filename = f"Output/0/image4.png"
# im = Image.open(filename)
# img = im.load()
# fig,axs = plt.subplots(2, 2)
#
# (width, height) = im.size
# data1 = np.zeros((height, width))
# data2 = np.zeros((height, width))
# data3 = np.zeros((height, width))
# data4 = np.zeros((height, width))
#
# def calc_center(data):
#     data = data >= COLOUR_THRESHOLD
#
#     sum = np.sum(np.sum(data))
#     if sum == 0: return (0, 0)
#     data = data / sum
#     dx = np.sum(data, 0)
#     dy = np.sum(data, 1)
#
#     cx = np.sum(dx * np.arange(width))
#     cy = np.sum(dy * np.arange(height))
#
#     return (cx, cy)
#
#
# for x in range(width):
#     for y in range(height):
#         c = img[(x, y)]
#         data1[y, x] = c[0]
#         data2[y, x] = c[1]
#         data3[y, x] = c[2]
#         data4[y, x] = c[1] * 1/3 + c[2] * 2/3
#
# radius = 5
# axs[0][0].imshow(data1)
# c0 = plt.Circle(calc_center(data1), radius, color="r")
# axs[0][0].add_patch(c0)
#
# axs[0][1].imshow(data2)
# c1 = plt.Circle(calc_center(data2), radius, color="r")
# axs[0][1].add_patch(c1)
#
# axs[1][0].imshow(data3)
# c2 = plt.Circle(calc_center(data3), radius, color="r")
# axs[1][0].add_patch(c2)
# #
# axs[1][1].imshow(data4)
# c3 = plt.Circle(calc_center(data4), radius, color="r")
# axs[1][1].add_patch(c3)
#
# # ax.add_patch(center)
# plt.show()

# exit(-1)

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

        data = np.zeros((height, width))

        for x in range(width):
            for y in range(height):
                c = img[(x, y)]
                data[y, x] = c[1] * 1/3 + c[2] * 2/3

        data = data >= COLOUR_THRESHOLD
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

            # x = max(-110, min(x, 110))
            # z = max(-110, min(z, 110))

            centers.append((x, y, z))
        else:
            centers.append((None, None, None))


    true_x = np.mean([p[0] for i, p in enumerate(centers) if i % 2 == 0 and not p[0] is None])
    true_y = height - np.mean([p[1] for p in centers if not p[1] is None])
    true_z = np.mean([p[2] for i, p in enumerate(centers) if i % 2 == 1 and not p[2] is None])

    if true_x is None or math.isnan(true_x):
        true_x = None
    if true_y is None or math.isnan(true_y):
        true_y = None
    if true_z is None or math.isnan(true_z):
        true_z = None

    # print(f"{image_index}: {centers} : {true_x}, {true_y}, {true_z}")
    print(f"Finished {image_index}")
    return (true_x, true_y, true_z)

new_position = [(0, 0, 0)] * LED_COUNT
def get_mean(position, i, index):
    current_value = position[i]
    new_value = 0

    if p[index] == None:
        sum = 0
        count = 0
        if i - 1 >= 0:
            value = position[i - 1][index]
            if not value is None:
                sum += value
                count += 1

        if i + 1 < LED_COUNT:
            value = position[i + 1][index]
            if not value is None:
                sum += value
                count += 1

        if count == 0:
            print(f"Bad: {i} : {index}")
            new_value = new_position[i-1][index]
        else:
            new_value = sum / count
    else:
        sum = position[i][index]
        count = 1

        if i - 1 >= 0:
            value = position[i - 1][index]
            if not value is None:
                sum += value
                count += 1

        if i + 1 < LED_COUNT:
            value = position[i + 1][index]
            if not value is None:
                sum += value
                count += 1

        new_value = sum / count

    x = new_position[i][0]
    y = new_position[i][1]
    z = new_position[i][2]

    if index == 0:
        x = new_value
    elif index == 1:
        y = new_value
    elif index == 2:
        z = new_value

    new_position[i] = (x, y, z)

position = [get_position(x) for x in range(LED_COUNT)]


for i, p in enumerate(position):
    get_mean(position, i, 0)
    get_mean(position, i, 1)
    get_mean(position, i, 2)

data = [x for x in zip(range(LED_COUNT), new_position)]

id = np.array([p[0] for p in data])
xs = np.array([p[1][0] for p in data])
ys = np.array([p[1][1] for p in data])
zs = np.array([p[1][2] for p in data])

y_offset = np.min(ys)
ys -= y_offset

with open("Output.pixels", "w") as file:
    for (id, x, y, z) in zip(id, xs, ys, zs):
        file.write(f"{id}: {x:.3f} {y:.3f} {z:.3f}\n")

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
