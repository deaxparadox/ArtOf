# nested `lambda` function


Like nested function definitions, lambda functions can reference variables from the containing scopes:

```python
>>> def make_incrementor(n):
...     return lambda x: x + n
... 
>>> f = make_incrementor(42)
>>> f(0)
42
>>> f(1)
43
```