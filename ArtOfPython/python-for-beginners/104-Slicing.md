# Slicing

*Slicing* allows you to obtain substring:

```python
>>> word[0:2]  # characters from position 0 (included) to 2 (excluded)
'Py'
>>> word[2:5]  # characters from position 2 (included) to 5 (excluded)
'tho'
```

- Slice indices have usefull defaults; an omitted first default to zero, an omitted second index defaults to the size of the strings being sliced.

```python
>>> word[:2]   # character from the beginning to position 2 (excluded)
'Py'
>>> word[4:]   # characters from position 4 (included) to the end
'on'
>>> word[-2:]  # characters from the second-last (included) to the end
'on'
```

- Note that the start is always included, and the end always excluded. This makes sure that `s[:i] + s[i:]` is always equal to `s`:

```python
>>> word[:2] + word[2:]
'Python'
>>> word[:4] + word[4:]
'Python'
```

- Out of range slice indexes are handled gracefully when used for slicing:

```python
>>> word[4:42]
'on'
>>> word[42:]
''
```

## Immutability

Python strings cannot be changed -- they are immutable. Therefore, assigning to an indexed position in the string results in an error:

```python
>>> word[0] = 'J'
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: 'str' object does not support item assignment
>>> 
>>> word[2:] = 'py'
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: 'str' object does not support item assignment
```

## Length of the string

- The built-in function `len()` returns the length of a string:

```python
>>> s = 'supercalifragilisticexpialidocious'
>>> len(s)
34
```