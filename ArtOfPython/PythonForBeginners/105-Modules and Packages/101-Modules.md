# Modules

Let say `mod.py` file which contains functions, variables, classes, etc, when you access these functions, or variables in another Python file say `main.py`, then `mod.py` if called module. 


Create a file `mod.py`, and defined a variable and a square function is it:

```py
# mod.py

i = 10

def square(x):
    return x * x
```

Now create another file `main.py`, import module `mod.py`, and access it attributes:

```py
# main.py

import mod

print(mod.i)

s = mod.square(10)
print(s)
```

```output
10
100
```