

import math

var_5 = 1
var_6 = 1.2
var_7 = 1
def func(var_8):
    global var_6
    global var_7
    for var_9 in range(1, var_8):
        var_10 = (var_6 + var_7)
        var_6 = var_7
        var_7 = var_10
    var_7
    return var_7
var_11 = func
def func(var_12):
    global var_5
    (2 * (var_5 + var_12))
    return (2 * (var_5 + var_12))
var_13 = func
var_14 = var_13(100)
var_15 = var_11(100)
print(var_15)

    