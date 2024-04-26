

import math
import std_lib as std

class Struct:
    def __init__(self, x, y, z, is_fixed_x=False, is_fixed_y=False, is_fixed_z=False, is_fixed_mx=False, is_fixed_my=False, is_fixed_mz=False):
        self.x = x
        self.y = y
        self.z = z
        self.is_fixed_x = is_fixed_x
        self.is_fixed_y = is_fixed_y
        self.is_fixed_z = is_fixed_z
        self.is_fixed_mx = is_fixed_mx
        self.is_fixed_my = is_fixed_my
        self.is_fixed_mz = is_fixed_mz

var_2 = Struct
def func(var_3):

    return var_3.x
var_4 = func
var_5 = var_2(x=1, z=3, y=2)
std.my_print(value=var_4(var_3=var_5))

    