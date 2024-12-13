import matplotlib.pyplot as plt
import re
import numpy as np

data = []
with open("Output.pixels", "r") as file:
    data = file.readlines()

data = [re.findall(r'(\d+): (-?\d+.\d+) (-?\d+.\d+) (-?\d+.\d+)', line) for line in data]

for p in data:
    print(p[0])
xs = np.array([float(p[0][1]) for p in data])
ys = np.array([float(p[0][2]) for p in data])
zs = np.array([float(p[0][3]) for p in data])

print(len(xs))
print(len(ys))
print(len(zs))

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
