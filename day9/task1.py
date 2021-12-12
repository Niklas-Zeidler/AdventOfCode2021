#%%
import numpy as np
# temp = np.loadtxt('./day9/sample.txt',str)
# input = np.zeros((5,10))
temp = np.loadtxt('./day9/input.txt',str)
input = np.zeros((100,100))

for k,col in enumerate(temp):
    for i,char in enumerate(col):
        input[k,i] = int(char)
# %%
matrix = np.zeros(shape=[input.shape[0]+2,input.shape[1]+2])
matrix = matrix -1
matrix[1:-1,1:-1] = input
risk_level = 0
for x in range(1,matrix.shape[0]-1):
    for y in range(1,matrix.shape[1]-1):
        current_element = matrix[x,y]
        neighbour_elements = np.array([matrix[x-1,y],matrix[x,y-1],matrix[x+1,y],matrix[x,y+1]])
        neighbour_elements = neighbour_elements[neighbour_elements != -1]
        if all(neighbour_elements>current_element):
            risk_level += current_element+1
print(risk_level)
