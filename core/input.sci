
a = 1
b = 1

out = for i in 1..100 {
    c = a + b
    a = b
    b = c
}

print(out)
