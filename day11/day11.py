#%% task 1
import numpy as np
from numpy.core.getlimits import _register_known_types
input = np.loadtxt('sample.txt',str)
# input = np.loadtxt('input.txt',str)
matrix = np.zeros((12,12))
for k,row in enumerate(input):
    for i,char in enumerate(row):
        matrix[k+1,i+1] = int(char)
def excite_neighbours(matrix,iter):
    row = iter[0]
    col = iter[1]
    matrix[row-1,col-1] += 1
    matrix[row,col-1] += 1
    matrix[row+1,col-1] += 1
    matrix[row-1,col+1] += 1
    matrix[row,col+1] += 1
    matrix[row+1,col+1] += 1
    matrix[row-1,col] += 1
    matrix[row+1,col] += 1
    return matrix
def null_boundaries(matrix):
    matrix[:,0] = 0
    matrix[:,-1] = 0
    matrix[0,:] = 0
    matrix[-1,:] = 0
    return matrix

    
nbr_of_days = 1
for day in range(nbr_of_days):
    matrix += 1
    matrix = null_boundaries(matrix)
    excited_elements = np.zeros((10,10))
    while np.any(matrix > 9):
        it = np.nditer(matrix, flags=['multi_index'])
        for x in it:
            if x > 9:
                matrix = excite_neighbours(matrix,it.multi_index)
                matrix = null_boundaries(matrix)
                excited_elements[it.multi_index[0]-1,it.multi_index[1]-1] += 1


# %%
