
unit m
unit kg
unit s

const N = kg*m/s^2

a = 1.2
b = 1


calculate_f = (f: num) => {
    for i in 1..f {
        c = a + b
        a = b
        b = c
    }
}

f = (f: num[N]) => {
    2N + f
}

fofrce = f(100N)

out = calculate_f(100)

print(out)
