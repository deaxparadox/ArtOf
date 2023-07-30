# Classes Can Intercept Python Operators

- *Operator overloading* lets objects coded with classes intercept and respond to operations that work on built-in types: addition, subtraction, qualification, and so on.

- *Operator overloading* lets object be more tightly integrated with Python's object model.

Here is a quick rundown of the main ideas behind overloading operators:

- Methods named with double underscores (__X__) are special hooks.
- Such methods are called automaticallly when instances appear in built-in operations.
- Classes may override most built-in type operations.
- There are no defaults for operation overloading methods, and none are required.
- Operators allow classes to integrate with Python's object.

## A Third Example

Three Special named attributes that Python will call automatically:

- `__init-_` is run when a new instance object is created: `self` is the new `ThirdClass` object.
- `__add__` is run when a `ThirdClass` instance apperas in `a + expression`
- `__str__` is run when an object is printed.

```py
class FirstClass:
    def setData(self, value):
        self.data = value
    def display(self):
        print(self.data) 

class SecondClass(FirstClass):
    def display(self):
        print("Current value = '%s'" % self.data)

class ThirdClass(SecondClass):                      # Inherited from SeconClass
    def __init__(self, value):                      # On "ThirdClass(value)"
        self.data = value 
    def __add__(self, other):                       # On "self + other"
        return ThirdClass(self.data + other)
    def __str__(self):                              # On "print(self)", "str()"
        return '[ThirdClass: %s]' % self.data
    def mul(self, other):                           # In-place change: named
        self.data *= other
```

```py
>>> a = ThirdClass('abc')
>>> a.display()
Current value = 'abc'
>>> 
>>> print(a)
[ThirdClass: abc]
>>> 
>>> b = a + 'xyz'
>>> b.display()
Current value = 'abcxyz'
>>> print(b)
[ThirdClass: abcxyz]
>>> 
>>> a.mul(3)
>>> print(a)
[ThirdClass: abcabcabc]
>>> 
```