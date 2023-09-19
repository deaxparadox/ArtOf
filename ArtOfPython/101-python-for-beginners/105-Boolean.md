# Boolean Types

Boolean types is a constant type, consisting of two object, *True* and *False*:


```python
>>> 
>>> t = True
>>> 
>>> f = False
>>> 
>>> print(t)
True
>>> print(f)
False
>>> 
```

## Boolean Operations -- and, or, not

These are the Boolean operations

- x or y: if x is true, then x, else y
- It return True if either of the condition (x/y) is True.

```python
>>> True or True
True
>>> True or False
True
>>> False or True
True
>>> False or False
False
```

- x and y: if x is false, then x, else y
- It return True if both of the condition (x,y) is True.

```python
>>> True or True
True
>>> True or False
False
>>> False or True
False
>>> False or False
False
```


- not x: if x is false, then `True`, else `False`
- if conversion a value to another.

```python
>>> not True
False
>>> not False 
True
```