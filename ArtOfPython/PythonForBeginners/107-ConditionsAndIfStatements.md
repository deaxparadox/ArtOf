# Conditions and `if` Statement

## Condition

Condition is a comparism operation which returns a *boolean type*. 

```python
>>> 1 == 1
True
>>> True == True
True
>>> True == False
False
>>> 'linux' == 'window'
False 
>>> 'linux' == 'linux'
True
```

Based of the condition, `if` statement execute the code block.

## `if` statement

```python
>>> if True:
...     print("It is true")
... else:
...     print("It is false")
... 
It is true
>>> 
```

```python
a = False
>>> if a:
...     print("It is true")
... else:
...     print("It is false")
... 
It is false
>>> 
```

```python
>>> response = 200
>>> if response == 200:
...     print('request was successfull')
... else:
...     print('request was unsuccessfull')
... 
request was successfull
>>> 
>>> # comparison
>>> response == 200
True
>>> 
```

```python
>>> x = int(input("Please enter an integer: "))
Please enter an integer: 42
>>> if x < 0:
...     x = 0
...     print('Negative changed to zero')
... elif x == 0:
...     print('Zero')
... elif x == 1:
...     print('Single')
... else:
...     print('More')
...
More
```

- *input()* function is used to take input from the user.
- *int()* function convert the string to integer. We will see move about it in *Type casting*.