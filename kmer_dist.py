import matplotlib.pyplot as plt
import pickle

with open("data/vec.pkl", "rb") as f:
    vec = pickle.load(f)

# Display
plt.scatter(range(len(vec)), vec, s=1, c='black', alpha=0.5, marker='o', edgecolors='none')
plt.savefig("data/scatter.png")
plt.show()
