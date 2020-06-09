import pandas as pd
import matplotlib.pyplot as plt

data = pd.read_csv("output.csv")
data = data.sort_values("proportion")
data = data.reset_index()
plt.scatter(x = data.index, y=data["proportion"])
plt.show()

print(list(data["proportion"][:30]))
