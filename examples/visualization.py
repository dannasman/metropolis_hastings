import matplotlib as mpl
from matplotlib import pyplot as plt
import numpy as np
import pandas as pd

df = pd.read_csv("data.txt", sep=" ")
x = df['x']
y = df['y']
spin = df['spin']
print(np.max(spin))

z = np.array(spin).reshape(np.max(x)+1, np.max(y)+1)

plt.imshow(z, cmap='Greys', interpolation=None, vmin=-1, vmax=1)
plt.colorbar()
plt.show()