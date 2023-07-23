# Slicing

While indexing is used to obtain individual characters, *slicing* allows you to obtain substring:

```python
>>> word = "Python"
>>> word[0:2]  # characters from position 0 (included) to 2 (excluded)
'Py'
>>> word[2:5]  # characters from position 2 (included) to 5 (excluded)
'tho'
```

- Slice indices have usefull defaults; an omitted first index default to zero, an omited second defaults to the size of the string being sliced.

```python
>>> word[:2]   # character from the beginning to position 2 (excluded)
'Py'
>>> word[4:]   # characters from position 4 (included) to the end
'on'
>>> word[-2:]  # characters from the second-last (included) to the end
'on'
```

- **Note how the start is always included, and the end always excluded. This make sure that `s[:i] + s[i:]` is always equal to `s`:

```python
>>> word[:2] + word[2:]
'Python'
>>> word[:4] + word[4:]
'Python'
```

Attempting to use an index t3hat is too large will result in an error:

```py
>>> word[42]  # the word only has 6 characters
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
IndexError: string index out of range
```

Out of range slice indexes are handled gracefulll when used for slicing:

```py
>>> word[4:42]
'on'
>>> word[42:]
''
```

Python string cannot be changed--they are `immutable`.

- Therefore, assigning to an indexed position in the string results in an error:

```py
>>> word[0] = 'J'
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: 'str' object does not support item assignment
>>> word[2:] = 'py'
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: 'str' object does not support item assignment
```

- If you need a different string, you should create a new one:

```py
>>> 'J' + word[1:]
'Jython'
>>> word[:2] + 'py'
'Pypy'
```

The built-in function len() returns the length of a string:

```py
>>> s = 'supercalifragilisticexpialidocious'
>>> len(s)
34
```