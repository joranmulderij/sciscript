
unit m
unit kg
unit s

N = kg*m/s^2
N_ = N

a = 1
b = 1


calculate_f = (f: num) => {
    for i in 1..f {
        c = a + b
        a = b
        b = c
    }
}

out = calculate_f(100)

print(out)
