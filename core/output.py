

import math

var_5 = (1 / 1)
var_6 = var_5
var_7 = 1
var_8 = 1
def func(var_9):
    global var_7
    global var_8
    for var_10 in range(1, var_9):
        var_11 = (var_7 + var_8)
        var_7 = var_8
        var_8 = var_11
    var_8
    return var_8
var_12 = func
var_13 = var_12(100)
print(var_13)

    