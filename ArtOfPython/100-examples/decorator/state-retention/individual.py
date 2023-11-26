def state_retention(func):
    i = 0
    def wrapper(*args, **kwargs):
        nonlocal i
        i += 1
        print(f"Counted from: {func.__name__}: {i} times")
        return func(*args, **kwargs)
    return wrapper

@state_retention
def f(): ...

@state_retention
def g(): ...

if __name__ == "__main__":
    f()
    f()
    g()
    for i in range(5): g()
    f()
    