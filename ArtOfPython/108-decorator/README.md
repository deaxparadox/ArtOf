# Decorator

Decorators are a way to add functionality to an existing function. This is called metaprogramming.

Basically, decorators are functions that call another function.

##### Example:

Here is a simple decorator example:

```py
def Decorator(func):
    def wrapper(*args, **kwargs):
        print("This is decorator")
        return func(*args, **kwargs)
    return wrapper

@Decorator
def hello_world():
    return "Hello World"

print(hello_world())
```

The output of the above example is:

```bash
This is decorator
Hello World
```


In the above code, we have defined decorator a `Decorator`:

```py
def Decorator(func):
    def wrapper(*args, **kwargs):
        print("This is decorator")
        return func(*args, **kwargs)
    return wrapper
```

and it is used, using `@` symbol:

```py
@Decorator
def func(): ...
```

and whenever a function `func()`, the decorator will be run automatically.


- [Timer Decorator Using Function](101-timer-decorators.md)
- [Timer Decorator Using Class](102-timer-decorators-using-clas.md)