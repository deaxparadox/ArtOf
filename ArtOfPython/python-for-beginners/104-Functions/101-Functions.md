# Functions

## Defining Functions

```py
def name():
    print("EveryWhereLinux")
```

```output
EeryWhereLinux
```

The keyword `def` used for defining function. It must be following by the function name nad the parenthesized, list of formal parameter which is optional in this case.

In this case we defined a function, it's just print the string `EveryWhereLinux`.

Now let's define a function that return the same string, and print it:

```py
def name():
    return "EveryWhereLinux"

s = name()
print(s)
```

It print the same output as preious example, but now `name()` function return a `string`, which we stored in variable `s`, and then use `print()` to display value of `s`.

We can combine last two step in one:

```py
def name():
    return "EveryWhereLinux"
 
print(name())
```

```output
EveryWhereLinux
```

You must be thinking why we are returning a value, if we can just print it. Returning is important, in cases where you want to put the return value into another function as input (or you want to print value and pass it as input)

Let's see example of that case:

```py
def name():
    return "EveryWhereLinux"

def hello(n):
    return "Hello " + n

s = name()
print(hello(s))
```

We return `EveryWhereLinux` from `name()` and pass it as input to `hello()`.

Example 1: In this example we are going to build a function that accept two numbers, and return addition:

```py
def add(a, b):
    return a + b

print(add(1,2))
```

```output
3
```

Like you can create n number of funtion using different different condition, statement, loop, function etc.