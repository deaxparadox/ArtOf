

def decorator(a0):
    def wrapper(*args):
        args = (23, 23, 2, 3, *args)
        return a0(*args)
    return wrapper

def take_parameters(msg: bool = False):
    if msg:
        print("This decorator function  takes messages")
    return decorator



@take_parameters('This decorator take message')
def func(*args): 
    return sum(args)


print(func(1, 2))