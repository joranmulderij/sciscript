def num(s):
    if isinstance(s, bool):
        return int(s)
    elif isinstance(s, int):
        return s
    elif isinstance(s, float):
        return s
    elif isinstance(s, str):
        try:
            return int(s)
        except ValueError:
            return float(s)
    else:
        return float(s)
    
def my_print(value):
    print(value)
    return value
    
def linspace(start, stop, n):
    step = (stop - start) / n
    return [start + i * step for i in range(n)]

def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0]]