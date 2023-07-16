import re
import matplotlib.pyplot as plt
from scipy.cluster.hierarchy import dendrogram
import pickle

# This script is to parse dendrogram string and draw the dendrogram

# Load the pickle file
with open('data/dendrogram.pkl', 'rb') as f:
    dendrogram_data = pickle.load(f)

with open('data/names.pkl', 'rb') as f:
    names = pickle.load(f)

# The input Dendrogram data
dendrogram_data = """
Dendrogram { steps: [Step { cluster1: 1, cluster2: 3, dissimilarity: 2.0, size: 2 }, Step { cluster1: 15, cluster2: 26, dissimilarity: 2.0, size: 2 },
Step { cluster1: 33, cluster2: 56, dissimilarity: 2.0, size: 3 }, Step { cluster1: 4, cluster2: 16, dissimilarity: 2.0, size: 2 }, Step { cluster1: 5,
cluster2: 10, dissimilarity: 2.0, size: 2 }, Step { cluster1: 9, cluster2: 37, dissimilarity: 2.0, size: 2 }, Step { cluster1: 28, cluster2: 60, dissim
ilarity: 2.0, size: 3 }, Step { cluster1: 14, cluster2: 21, dissimilarity: 2.0, size: 2 }, Step { cluster1: 51, cluster2: 59, dissimilarity: 2.0, size:
 3 }, Step { cluster1: 36, cluster2: 62, dissimilarity: 2.0, size: 4 }, Step { cluster1: 34, cluster2: 58, dissimilarity: 2.0, size: 4 }, Step { cluste
r1: 40, cluster2: 66, dissimilarity: 2.0, size: 5 }, Step { cluster1: 32, cluster2: 45, dissimilarity: 2.0, size: 2 }, Step { cluster1: 39, cluster2: 4
2, dissimilarity: 2.0, size: 2 }, Step { cluster1: 41, cluster2: 67, dissimilarity: 2.0, size: 6 }, Step { cluster1: 43, cluster2: 70, dissimilarity: 2
.0, size: 7 }, Step { cluster1: 55, cluster2: 71, dissimilarity: 2.0, size: 8 }, Step { cluster1: 47, cluster2: 54, dissimilarity: 2.0, size: 2 }, Step
 { cluster1: 13, cluster2: 18, dissimilarity: 3.0, size: 2 }, Step { cluster1: 2, cluster2: 57, dissimilarity: 3.0, size: 3 }, Step { cluster1: 11, clu
ster2: 19, dissimilarity: 3.0, size: 2 }, Step { cluster1: 22, cluster2: 68, dissimilarity: 3.0, size: 3 }, Step { cluster1: 30, cluster2: 61, dissimil
arity: 3.0, size: 3 }, Step { cluster1: 46, cluster2: 76, dissimilarity: 3.5, size: 3 }, Step { cluster1: 17, cluster2: 65, dissimilarity: 4.0, size: 5
 }, Step { cluster1: 23, cluster2: 50, dissimilarity: 4.0, size: 2 }, Step { cluster1: 29, cluster2: 81, dissimilarity: 7.75, size: 3 }, Step { cluster
1: 44, cluster2: 74, dissimilarity: 15.5, size: 3 }, Step { cluster1: 0, cluster2: 52, dissimilarity: 17.25, size: 2 }, Step { cluster1: 8, cluster2: 4
8, dissimilarity: 27.0, size: 2 }, Step { cluster1: 38, cluster2: 84, dissimilarity: 44.5, size: 3 }, Step { cluster1: 6, cluster2: 7, dissimilarity: i
nf, size: 2 }, Step { cluster1: 12, cluster2: 87, dissimilarity: inf, size: 3 }, Step { cluster1: 20, cluster2: 88, dissimilarity: inf, size: 4 }, Step
 { cluster1: 63, cluster2: 89, dissimilarity: inf, size: 6 }, Step { cluster1: 24, cluster2: 90, dissimilarity: inf, size: 7 }, Step { cluster1: 25, cl
uster2: 91, dissimilarity: inf, size: 8 }, Step { cluster1: 75, cluster2: 92, dissimilarity: inf, size: 11 }, Step { cluster1: 27, cluster2: 93, dissim
ilarity: inf, size: 12 }, Step { cluster1: 31, cluster2: 94, dissimilarity: inf, size: 13 }, Step { cluster1: 35, cluster2: 95, dissimilarity: inf, siz
e: 14 }, Step { cluster1: 80, cluster2: 96, dissimilarity: inf, size: 19 }, Step { cluster1: 78, cluster2: 97, dissimilarity: inf, size: 22 }, Step { c
luster1: 69, cluster2: 98, dissimilarity: inf, size: 24 }, Step { cluster1: 83, cluster2: 99, dissimilarity: inf, size: 27 }, Step { cluster1: 77, clus
ter2: 100, dissimilarity: inf, size: 30 }, Step { cluster1: 79, cluster2: 101, dissimilarity: inf, size: 33 }, Step { cluster1: 85, cluster2: 102, diss
imilarity: inf, size: 35 }, Step { cluster1: 49, cluster2: 103, dissimilarity: inf, size: 36 }, Step { cluster1: 82, cluster2: 104, dissimilarity: inf,
 size: 39 }, Step { cluster1: 64, cluster2: 105, dissimilarity: inf, size: 42 }, Step { cluster1: 86, cluster2: 106, dissimilarity: inf, size: 45 }, St
ep { cluster1: 53, cluster2: 107, dissimilarity: inf, size: 46 }, Step { cluster1: 73, cluster2: 108, dissimilarity: inf, size: 48 }, Step { cluster1:
72, cluster2: 109, dissimilarity: inf, size: 56 }], observations: 56 }
"""

# parse the string
# remove the first and last line





# Show the dendrogram plot
plt.show()
plt.show()
plt.savefig("data/dendrogram.png")
