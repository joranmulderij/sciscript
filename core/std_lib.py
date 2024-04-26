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
    
def my_print(*args):
    print(*args)
    if len(args) >= 1:
        return args[0]
    else:
        return None
    
def linspace(start, stop, num):
    step = (stop - start) / num
    return [start + i * step for i in range(num)]