# Package

Python package is a collection of module.

Now we are going to see how to create python package.


Steps for creating packages:

- create a directory `pkg`.
- creating `__init__.py` in `pkg`, which tells python to treat this directory as package.
- create another file `mod.py` inside `pkg`, and create a variable and a function in it.

```bash
.
├── main.py
├── pkg
│   ├── __init__.py
│   └── mod.py

```

```py
# mod.py

i = 10

def square(x): 
    return x * x
```

```py
# main.py

from pkg import mod

print(mod.i)

s = mod.square(5)
print(s)
```

```output
10
25
```