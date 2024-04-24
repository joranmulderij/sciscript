

import math

var_5 = (1 / 1)
var_6 = 1
var_7 = 1
def func(var_8):
    global var_5
    global var_6
    global var_7
    for var_9 in range(1, var_8):
        var_10 = (var_6 + var_7)
        var_6 = var_7
        var_7 = var_10
    var_7
    return var_7
var_11 = func
var_12 = var_11(100)
print(var_12)

    