# try-except-finally

The `finally` in `try-except` statement block used to cleaning or running the final code. Either of the block `try` or `except` runs, `finally` block will run in the end.

```py
a = 1
b = 2
d = 4


try: print(a)
except: print(b)
finally: print(d)

# output
# 1
# 4
```


```py
b = 2
d = 4


try: print(a)
except: print(b)
finally: print(d)

# output
# 2
# 4
```