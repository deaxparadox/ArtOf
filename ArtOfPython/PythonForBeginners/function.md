# functions

## Defining Functions

We can create a function that writes the Fibonacci series to an arbitrary boundary:


```python
def fib(n):
    # print a Fibonacci series up to n.
    a = 0
    b = 1
    while a < n:
        print(a, end=" ")
        a = b
        b = a + b 
    print()

# executing functions.
fib(10)

# output
0 1 2 4 8 
```

- The keyword `def` introduces a function *definition*. It must be followed by the function name and the parenteized list of romat parameters. The statements that form the body of the function start at the next line, and must be intended.


## return statements


It is simple to write a function that returns a list of the numbers of the fibonacci series, instead of printing it:

```python

def fib(n):
    # print a Fibonacci series up to n.
    result = []
    a = 0
    b = 1
    while a < n:
        result.append(a)
        a = b
        b = a + b 
    return result

print(fib(10))

# output
[0, 1, 2, 4, 8]
```

- The `return` statement returns witha value from a function. return without an expression argument returns None. Falling off the end of a function also returns None.