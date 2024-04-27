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

fn get_element_length(n1: Node, n2: Node): num {
    let dx = n1.x - n2.x
    let dy = n1.y - n2.y
    let dz = n1.z - n2.z
    sqrt(dx*dx + dy*dy + dz*dz)
}

fn get_element_orientation(dx: num, dy: num, dz: num) {
    let L = sqrt(dx*dx + dy*dy + dz*dz)
    let x_loc = [dx/L, dy/L, dz/L]
    let theta = atan2(dz, dx)
    let x = cos(theta + pi / 2)
    let y = sin(theta + pi / 2)
    let z_loc = [x, 0, z]
    let y_loc = cross(x_loc, z_loc)
}

fn get_element_R(n1: Node, n2: Node): list[list[num]] {
    let dx = n1.x
}

struct ElementProperties {
    E: num
    R: num
    T: num
    poissons_ratio: num
    density: num

    fn getG() {
        E / (2 * (1 + poissons_ratio))
    }

    fn getA() {
        0.25 * pi * (pow(2*R, 2) - pow((2*R - 2 * T), 2))
    }

    fn getRInside() {
        R - T
    }

    fn getI() {
        (pow(R, 4) - pow(getRInside(), 4)) * pi / 4
    }
}

struct Element {
    node1: num
    node2: num

    fn get_local_element_stiffness_matrix(nodes: map[num, Node], properties: ElementProperties): list[list[num]] {
        let n1 = nodes[node1]
        let n2 = nodes[node2]
        let L = get_element_length(n1, n2)
        let L3 = pow(L, 3)

        let J = 2 * properties.getI()
        let EA = properties.E * properties.getA()
        let GJ = properties.getG() * J
        let GJpL = GJ / L
        let EApL = EA / L
        let EI = properties.getI() * properties.E
        let EGIpL3 = EI / L3
        [
            [EApL, 0, 0, 0, 0, 0, -EApL, 0, 0, 0, 0, 0],
            [0, 12 * EGIpL3, 0, 0, 0, 6 * EGIpL3, 0, -12 * EGIpL3, 0, 0, 0, 6 * EGIpL3],
            [0, 0, 12 * EGIpL3, 0, -6 * EGIpL3, 0, 0, 0, -12 * EGIpL3, 0, -6 * EGIpL3, 0],
            [0, 0, 0, GJpL, 0, 0, 0, 0, 0, -GJpL, 0, 0],
            [0, 0, -6 * EGIpL3, 0, 4 * EGIpL3, 0, 0, 0, 6 * EGIpL3, 0, 2 * EGIpL3, 0],
            [0, 6 * EGIpL3, 0, 0, 0, 4 * EGIpL3, 0, -6 * EGIpL3, 0, 0, 0, 2 * EGIpL3],
            [-EApL, 0, 0, 0, 0, 0, EApL, 0, 0, 0, 0, 0],
            [0, -12 * EGIpL3, 0, 0, 0, -6 * EGIpL3, 0, 12 * EGIpL3, 0, 0, 0, -6 * EGIpL3],
            [0, 0, -12 * EGIpL3, 0, 6 * EGIpL3, 0, 0, 0, 12 * EGIpL3, 0, 6 * EGIpL3, 0],
            [0, 0, 0, -GJpL, 0, 0, 0, 0, 0, GJpL, 0, 0],
            [0, 0, -6 * EGIpL3, 0, 2 * EGIpL3, 0, 0, 0, 6 * EGIpL3, 0, 4 * EGIpL3, 0],
            [0, 6 * EGIpL3, 0, 0, 0, 2 * EGIpL3, 0, -6 * EGIpL3, 0, 0, 0, 4 * EGIpL3],
        ]
    }
}

struct Model {
    nodes: map[num, Node]
    elements: list[Element]

    fn f() {
        elements[0].node1
    }
}

let props = ElementProperties(E = 1, R = 2, T = 1, poissons_ratio = 0.3, density = 1)

let model = Model(nodes = {1: Node(1,2,3)}, elements = [Element(1,2)])


