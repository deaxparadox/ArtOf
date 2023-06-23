<!-- match statement -->

# `match` statement

A `match` statement takes an expression and compares its value to successive patterns given as one or more case blocks.

```py
count = 0
match count:
    case 0:
        print('Count is zero')
    case _:
        print("Count is non-zero")
```

```output
Count is zero
```