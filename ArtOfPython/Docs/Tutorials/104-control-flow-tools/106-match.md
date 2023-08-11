# `match` Statements

A `match` statement takes an expression and compares its value to successive patterns as one or more case blocks. 

```python
status = 200

match status:
    case 200:
        print("Successfull")
    case 300:
        print("Redirected")
    case _:
        print("Unknown!")

# output
SuccessFull
```

- You can combine several literals in a single pattern using `|` ("or"):

```python
status = 302

match status:
    case 200:
        print("Successfull")
    case 300 | 301 | 302:
        print("Redirected")
    case _:
        print("Unknown!")
```