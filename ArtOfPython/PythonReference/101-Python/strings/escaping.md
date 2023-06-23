
## `\` is used to escape characters

`\` for escaping single quotes

```python   
# escaping quote
>>> 'Isn\'t they said'
"Isn't they said"
```

```python
# not escaping quotes
>>> 'Isn't they said'
  File "<stdin>", line 1
    'Isn't they said'
         ^
SyntaxError: invalid syntax
>>> 
```

- We are getting error on not escaping quotes because python thing String is ending after character *n*. 

`\n` means newline

- `\n` add a newline to the string, so when we use *print()* function, newline get formatted in the terminal.

```python
>>> s = 'First line.\nSecone line'
>>> s
'First line.\nSecone line'
>>> print(s)
First line.
Secone line
>>> 
```

- `r''`, If you don't want characters prefaced by `\` to be interpreted as special characters, you can use *raw strings* by adding an `r` before the first quote:

```python
>>> print('\some\name')  # here \n means newline!
\some
ame
>>> print(r'\some\name')  # note the r before the quote
\some\name
```
