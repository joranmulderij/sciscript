

struct Node {
    x: num
    y: num
    z: num
    is_fixed_x: bool = false
    is_fixed_y: bool = false
    is_fixed_z: bool = false
    is_fixed_mx: bool = false
    is_fixed_my: bool = false
    is_fixed_mz: bool = false
}

node = Node(1, 2, 3)
print(getX(node))
