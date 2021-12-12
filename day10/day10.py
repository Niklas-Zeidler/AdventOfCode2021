#%% task 1
import numpy as np
from numpy.core.getlimits import _register_known_types
# input = np.loadtxt('./day10/sample.txt',str)
input = np.loadtxt('./day10/input.txt',str)
# %%
def char_is_opening(char):
    set = np.array(['{','[','(','<'])
    if any(char == set):
        return True
    else:
        return False
def open_or_closing(character,a,b,count):
    if character == a:
        count += 1
    if character == b:
        count -= 1
    return count
def closing_fits(temp_string,char):
    closing_needed = appropriate_closing.get(temp_string[-1])
    if char == closing_needed:
        return True
    else:
        return False
delimiters = np.zeros(4)
illegal_lines = []
scores = { 
    ')' : 3,
    ']' : 57,
    '}' : 1197,
    '>' : 25137
}
appropriate_closing = {
    '(' : ')',
    '[' : ']',
    '{' : '}',
    '<' : '>'
}
total_score = 0
for k,row in enumerate(input):
    delimiters[:] = 0
    temp_string = ''
    faulty_line = False
    for character in row:
        if char_is_opening(character):
            temp_string += character
        else:
            if closing_fits(temp_string,character):
                temp_string = temp_string[:-1]
            else:
                total_score += scores.get(character)
                faulty_line = True
                break
    print(temp_string)
    # print(delimiters)
        # 
print('total score :',total_score)
# %%
def char_is_opening(char):
    set = np.array(['{','[','(','<'])
    if any(char == set):
        return True
    else:
        return False
def open_or_closing(character,a,b,count):
    if character == a:
        count += 1
    if character == b:
        count -= 1
    return count
def closing_fits(temp_string,char):
    closing_needed = appropriate_closing.get(temp_string[-1])
    if char == closing_needed:
        return True
    else:
        return False
delimiters = np.zeros(4)
illegal_lines = []
scores = { 
    ')' : 1,
    ']' : 2,
    '}' : 3,
    '>' : 4
}
appropriate_closing = {
    '(' : ')',
    '[' : ']',
    '{' : '}',
    '<' : '>'
}
total_score = np.array([])
for k,row in enumerate(input):
    delimiters[:] = 0
    temp_string = ''
    faulty_line = False
    temp_score = 0
    for character in row:
        if char_is_opening(character):
            temp_string += character
        else:
            if closing_fits(temp_string,character):
                temp_string = temp_string[:-1]
            else:
                # total_score += scores.get(character)
                faulty_line = True
                break
    # print(temp_string)
    if (not faulty_line) & (len(temp_string) >1):
        # print(temp_string)
        for char in temp_string[::-1]:
            closing_needed = appropriate_closing.get(char)
            temp_score = temp_score*5 + scores.get(closing_needed)
        total_score =np.append(total_score,temp_score)
print(total_score)
print(np.median(total_score))
        

# %%
