## Slicing a strings

To create a substring from a string, you can use slicing. You can return a range of characters by using slicing:

Syntax for slcing is the start index and the end index, separated by a colon, to return a part of the string.

To create a substring from index `2` to index `5`:

```py
>>> print(a)
Hello, Python!
>>> 
>>> a[2:5]
'llo'
```

To slice from the start, leave out the start index, the range will start at the first character:

```py
>>> a[:5]
'Hello'
>>> 
```

To slice to the end, leave out hte end index, and range will go to the end:

```py
>>> a[2:]
'llo, Python!'
>>> 
```

You can also use negative indexes to start the slice from the end of the string:

```py
>>> a[-5:-2]
'tho'
>>> 
```

[next](105-Number.md)