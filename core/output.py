

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
def func(var_3, var_4):
    var_5 = (var_3.x - var_4.x)
    var_6 = (var_3.y - var_4.y)
    var_7 = (var_3.z - var_4.z)
    return math.sqrt(value=(((var_5 * var_5) + (var_6 * var_6)) + (var_7 * var_7)))
var_8 = func

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
    
        return (((math.pow(base=self.R, exp=4) - math.pow(exp=4, base=self.getRInside())) * math.pi) / 4)


var_9 = Struct

class Struct:
    def __init__(self, node1, node2):
        self.node1 = node1
        self.node2 = node2

    def get_local_element_stiffness_matrix(self, var_10, var_11):
        var_12 = var_10[self.node1]
        var_13 = var_10[self.node2]
        var_14 = var_8(var_4=var_13, var_3=var_12)
        var_15 = math.pow(exp=3, base=var_14)
        var_16 = (2 * var_11.getI())
        var_17 = (var_11.E * var_11.getA())
        var_18 = (var_11.getG() * var_16)
        var_19 = (var_18 / var_14)
        var_20 = (var_17 / var_14)
        var_21 = (var_11.getI() * var_11.E)
        var_22 = (var_21 / var_15)
        return [[var_20, 0, 0, 0, 0, 0, -var_20, 0, 0, 0, 0, 0], [0, (12 * var_22), 0, 0, 0, (6 * var_22), 0, (-12 * var_22), 0, 0, 0, (6 * var_22)], [0, 0, (12 * var_22), 0, (-6 * var_22), 0, 0, 0, (-12 * var_22), 0, (-6 * var_22), 0], [0, 0, 0, var_19, 0, 0, 0, 0, 0, -var_19, 0, 0], [0, 0, (-6 * var_22), 0, (4 * var_22), 0, 0, 0, (6 * var_22), 0, (2 * var_22), 0], [0, (6 * var_22), 0, 0, 0, (4 * var_22), 0, (-6 * var_22), 0, 0, 0, (2 * var_22)], [-var_20, 0, 0, 0, 0, 0, var_20, 0, 0, 0, 0, 0], [0, (-12 * var_22), 0, 0, 0, (-6 * var_22), 0, (12 * var_22), 0, 0, 0, (-6 * var_22)], [0, 0, (-12 * var_22), 0, (6 * var_22), 0, 0, 0, (12 * var_22), 0, (6 * var_22), 0], [0, 0, 0, -var_19, 0, 0, 0, 0, 0, var_19, 0, 0], [0, 0, (-6 * var_22), 0, (2 * var_22), 0, 0, 0, (6 * var_22), 0, (4 * var_22), 0], [0, (6 * var_22), 0, 0, 0, (2 * var_22), 0, (-6 * var_22), 0, 0, 0, (4 * var_22)]]


var_23 = Struct

class Struct:
    def __init__(self, nodes, elements):
        self.nodes = nodes
        self.elements = elements

    def f(self, ):
    
        return self.elements[0].node1


var_24 = Struct
var_25 = var_23(node2=2, node1=1)
var_26 = var_24(elements={1: var_2(x=1, z=3, y=2)}, nodes=[var_23(node1=1, node2=2)])

    