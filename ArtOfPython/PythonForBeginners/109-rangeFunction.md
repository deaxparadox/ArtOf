# `range()` functions

If you do need to iterate over a sequnce of numbers, the built-in function `range()`, comes in handy, It generates arithmetic progressions:

```python
>>> for i in range(5):
...     print(i)
... 
0
1
2
3
4
>>> 
```

- By default range count from 0, till the provided number.

If you want to count from another number, you provided *start* and *stop* indices:

```python
>>> for i in range(2, 5):
...     print(i)
... 
2
3
4
>>> 
```