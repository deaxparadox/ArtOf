def Decorator(func):
    def wrapper(*args, **kwargs):
        print("This is decorator")
        return func(*args, **kwargs)
    return wrapper

@Decorator
def hello_world():
    return "Hello World"

print(hello_world())