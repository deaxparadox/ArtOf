# Scopes

## `global` scope

Variables that are declare outside of functions are called *global variable*:

```py
g = "I am global"

def func():
    print(g)
    return 1

print(func())
```

```output
I am global 
1
```


### `local` scope

Variables that are declare inside of functions are called *local variable*.

```py
g = "I am global"

def func():
    l = "I am local"
    print(l)
    print(g)
    return 1

print(func())


# If you try to access `l` outside of function
# NameError will be raised
print(l)
```