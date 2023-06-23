
## Conacatenation `+`

```python
>>> s = 'un' + 'ium'
>>> s
'unium'
>>> print(s)
unium
>>> 
```

- Two or more *string literals* (i.e. the ones enclosed between quotes) next to each other automatically concatenated:

```python
>>> 
>>> 'every' 'where' 'linux'
'everywherelinux'
>>> 
```

- You can also put several strings within parentheses to have them joined together:

```python
>>> text = ('Put several strings within parentheses '
...     'to have them joined together.')
>>> text
'Put several strings within parentheses to have them joined together.'
```

- This only works with two literals though, not with variables or expressions:

```python
prefix = 'Py'
prefix 'thon'  # can't concatenate a variable and a string literal
  File "<stdin>", line 1
    prefix 'thon'
           ^^^^^^
SyntaxError: invalid syntax
('un' * 3) 'ium'
  File "<stdin>", line 1
    ('un' * 3) 'ium'
               ^^^^^
SyntaxError: invalid syntax
```

- If you want to concatenated variables or variables and a litrals, use `+`:

```python
>>> prefix = "Py"
>>> prefix + "thon"
'Python'
```

