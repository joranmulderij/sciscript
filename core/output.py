

import math
import std_lib as std
import numpy as np


class Struct:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z



var_2 = Struct

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



var_3 = Struct
def func(var_4, var_5):
    var_6 = (var_4.x - var_5.x)
    var_7 = (var_4.y - var_5.y)
    var_8 = (var_4.z - var_5.z)
    return math.sqrt(value=(((var_6 * var_6) + (var_7 * var_7)) + (var_8 * var_8)))
var_9 = func
def func(var_10, var_11, var_12):
    global var_2
    var_13 = math.sqrt(value=(((var_10 * var_10) + (var_11 * var_11)) + (var_12 * var_12)))
    var_14 = np.matrix([[(var_10 / var_13), (var_11 / var_13), (var_12 / var_13)]])
    var_15 = math.atan2(a=var_12, b=var_10)
    var_16 = math.cos(value=(var_15 + (math.pi / 2)))
    var_17 = math.sin(value=(var_15 + (math.pi / 2)))
    var_18 = np.matrix([[var_16, 0, var_17]])
    var_19 = np.cross(b=var_18, a=var_14)
    return var_2(y=var_19, z=var_18, x=var_14)
var_20 = func
def func(var_21, var_22):
    var_23 = (var_21.x - var_22.x)
    var_24 = (var_21.y - var_22.y)
    var_25 = (var_21.z - var_22.z)
    var_26 = var_20(var_11=var_24, var_10=var_23, var_12=var_25)
    var_27 = var_26.x
    var_28 = var_26.y
    var_29 = var_26.z
    var_30 = np.matrix([[1, 0, 0]])
    var_31 = np.matrix([[0, 1, 0]])
    var_32 = np.matrix([[0, 0, 1]])
    var_33 = np.matrix([[(var_30 * var_27), (var_30 * var_28), (var_30 * var_29), 0, 0, 0, 0, 0, 0, 0, 0, 0], [(var_31 * var_27), (var_31 * var_28), (var_31 * var_29), 0, 0, 0, 0, 0, 0, 0, 0, 0], [(var_32 * var_27), (var_32 * var_28), (var_32 * var_29), 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, (var_30 * var_27), (var_30 * var_28), (var_30 * var_29), 0, 0, 0, 0, 0, 0], [0, 0, 0, (var_31 * var_27), (var_31 * var_28), (var_31 * var_29), 0, 0, 0, 0, 0, 0], [0, 0, 0, (var_32 * var_27), (var_32 * var_28), (var_32 * var_29), 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, (var_30 * var_27), (var_30 * var_28), (var_30 * var_29), 0, 0, 0], [0, 0, 0, 0, 0, 0, (var_31 * var_27), (var_31 * var_28), (var_31 * var_29), 0, 0, 0], [0, 0, 0, 0, 0, 0, (var_32 * var_27), (var_32 * var_28), (var_32 * var_29), 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, (var_30 * var_27), (var_30 * var_28), (var_30 * var_29)], [0, 0, 0, 0, 0, 0, 0, 0, 0, (var_31 * var_27), (var_31 * var_28), (var_31 * var_29)], [0, 0, 0, 0, 0, 0, 0, 0, 0, (var_32 * var_27), (var_32 * var_28), (var_32 * var_29)]])
    return var_33
var_34 = func

class Struct:
    def __init__(self, E, R, T, poissons_ratio, density):
        self.E = E
        self.R = R
        self.T = T
        self.poissons_ratio = poissons_ratio
        self.density = density

    def getG(self, ):
    
        return (self.E / (2 * (1 + self.poissons_ratio)))
    def getA(self, ):
    
        return ((0.25 * math.pi) * (math.pow(exp=2, base=(2 * self.R)) - math.pow(exp=2, base=((2 * self.R) - (2 * self.T)))))
    def getRInside(self, ):
    
        return (self.R - self.T)
    def getI(self, ):
    
        return (((math.pow(base=self.R, exp=4) - math.pow(base=self.getRInside(), exp=4)) * math.pi) / 4)


var_35 = Struct

class Struct:
    def __init__(self, node1, node2):
        self.node1 = node1
        self.node2 = node2

    def get_local_element_stiffness_matrix(self, var_36, var_37):
        var_38 = var_36[self.node1]
        var_39 = var_36[self.node2]
        var_40 = var_9(var_5=var_39, var_4=var_38)
        var_41 = math.pow(exp=3, base=var_40)
        var_42 = (2 * var_37.getI())
        var_43 = (var_37.E * var_37.getA())
        var_44 = (var_37.getG() * var_42)
        var_45 = (var_44 / var_40)
        var_46 = (var_43 / var_40)
        var_47 = (var_37.getI() * var_37.E)
        var_48 = (var_47 / var_41)
        var_49 = np.matrix([[var_46, 0, 0, 0, 0, 0, -var_46, 0, 0, 0, 0, 0], [0, (12 * var_48), 0, 0, 0, (6 * var_48), 0, (-12 * var_48), 0, 0, 0, (6 * var_48)], [0, 0, (12 * var_48), 0, (-6 * var_48), 0, 0, 0, (-12 * var_48), 0, (-6 * var_48), 0], [0, 0, 0, var_45, 0, 0, 0, 0, 0, -var_45, 0, 0], [0, 0, (-6 * var_48), 0, (4 * var_48), 0, 0, 0, (6 * var_48), 0, (2 * var_48), 0], [0, (6 * var_48), 0, 0, 0, (4 * var_48), 0, (-6 * var_48), 0, 0, 0, (2 * var_48)], [-var_46, 0, 0, 0, 0, 0, var_46, 0, 0, 0, 0, 0], [0, (-12 * var_48), 0, 0, 0, (-6 * var_48), 0, (12 * var_48), 0, 0, 0, (-6 * var_48)], [0, 0, (-12 * var_48), 0, (6 * var_48), 0, 0, 0, (12 * var_48), 0, (6 * var_48), 0], [0, 0, 0, -var_45, 0, 0, 0, 0, 0, var_45, 0, 0], [0, 0, (-6 * var_48), 0, (2 * var_48), 0, 0, 0, (6 * var_48), 0, (4 * var_48), 0], [0, (6 * var_48), 0, 0, 0, (2 * var_48), 0, (-6 * var_48), 0, 0, 0, (4 * var_48)]])
        return var_49


var_50 = Struct

class Struct:
    def __init__(self, nodes, elements):
        self.nodes = nodes
        self.elements = elements

    def f(self, ):
    
        return self.elements[0].node1


var_51 = Struct
var_52 = var_35(poissons_ratio=0.3, R=2, T=1, E=1, density=1)
var_53 = var_51(nodes={1: var_3(x=1, y=2, z=3)}, elements=[var_50(node2=2, node1=1)])
var_54 = var_53.nodes[1]
std.my_print(value=var_54.is_fixed_x)

    