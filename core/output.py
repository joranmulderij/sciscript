

import math
import std_lib as std

var_5 = 1
var_6 = 1
var_7 = 1
def func(var_8):
    global var_6
    global var_7
    for var_9 in range(1, var_8):
        var_10 = (var_6 + var_7)
        var_6 = var_7
        var_7 = var_10
    return var_7
var_11 = func
def func(var_12):
    global var_5

    return (2 * (var_5 + var_12))
var_13 = func
var_14 = var_13(100)
var_15 = var_11(100)
print(var_15)
var_16 = [1, 2]
print(var_16[0], 2, 3, 2, 4)
print(1)
def func(var_17, *var_18):

    return print(var_18[0], var_18[1], var_17)
var_19 = func
var_20 = var_19(1, 2, 3)
var_20 = None
var_21 = 1
var_21 = False
print(std.linspace(10, 100, 90))
    