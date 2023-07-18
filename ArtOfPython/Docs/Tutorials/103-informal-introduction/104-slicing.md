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