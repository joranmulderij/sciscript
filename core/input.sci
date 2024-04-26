
unit m
unit kg
unit s

const N = kg*m/s^2

a = 1
b = 1

calculate_f = (f: num) => {
    for i in 1..f {
        c = a + b
        a = b
        b = c
    }
}

f = (F: num[N]) => 2N + F

F = f(100N)

out = calculate_f(100)

print(out)

let x: list[num] = [1, 2]
print(x[0], 2, 3, 2, 4)

print(1)

fn f (a: num, args: list[num]) {
    print(args[0], args[1], a)
}

let a = f(1, 2, 3)
a = null

let a: any = 1
a = false

print(linspace(10, 100, 90))



//struct Point {
//    x: num
//    y: num
//    
//    constructor(x: num, y: num) {
//        this.x = x
//        this.y = y
//    }
//
//    fn print() {
//        print(this.x, this.y)
//    }
//}

//fn plot(x: vec[num], y: vec[num], line_width: num, color: str = "red", args: vec[any]) {
//    print("plotting")
//}

//plot(x, y=y, line_width=2)
