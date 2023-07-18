# Indexing

Strings can be *indexed* (subscripted), with the first character having index 0. There is no separate character type; a character is simple a string of size one:

```python
>>> word = 'Python'
>>> word[0]  # character in position 0
'P'
>>> word[5]  # character in position 5
'n'
```

- Indices maty also be negative numbers to start counting from the right:

```python
>>> word[-1]  # last character
'n'
>>> word[-2]  # second-last character
'o'
>>> word[-6]
'P'
```

- **Note that since -0 is the same as 0, negative indices start from -1**.
