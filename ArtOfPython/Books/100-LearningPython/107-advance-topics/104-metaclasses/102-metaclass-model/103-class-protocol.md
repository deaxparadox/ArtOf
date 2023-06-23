# Class Statement Protocol

When Python reaches a class statement, it runs its nested blog of code of create its attributes--all the names assigned at the top level of the nested code block generate attributes in the resulting clas object. 

- These  names are usually emthod functions created by nested *defs*, but they can also be arbitrary attributes assigned to create calss data shared by all instances

Python follows a standard protocol to make this happend: at the *end of a class statement*, and after running al its nested code in a namespace dictionary corresponding ot the calss's local scope, PYthon calls the *type* object to create the *class* object like this:

```python
class = type(classname, superclasses, attributedict)
```

- The *type* object in turns defines a **__call__** operator overloading method that runs two other methods when the **type** object is called.

```python
type.__new__(typeclass, classname, superclasses, attributedict)
type.__init__(class, classname, superclasses, attributedict)
```

The **__new__** method creates and returns the new **class** object, and then the **__init__** method initializes the newly created object.

- these are the hooks that metaclass subclasses of **type** generally use to customize classes.

- for Example, given a class definition like the following for **Spam**:

```python
In [26]: class Eggs:
    ...:     ...
    ...: 

In [27]: def Spam(Eggs):
    ...:     data = 1
    ...:     def meth(self, arg):
    ...:         return self.data + arg
    ...: 

In [28]: 
```

- Pytho nwill internally run the nested code block to create two attributes of the class (**data** and **meth**), and then call the **type** object to generate the **class** object at the end of the **class** statement:

```python
Spam = type('Spam', (Eggs,), {'data': 1, 'meth': meth, '__module__': '__main__'})
```

- youc al call **type** this way yourself to create a class dyncamicall--albiet here with a fabricated method function and empty superclasses type:

```python
>>> x = type('Spam', (), {'data': 1, 'meth': (lambda x, y: x.data + y)})
>>> i = x()
>>> x, i
(<class '__main__.Spam'>, <__main__.Spam object at 0x029E7780>)
>>> i.data, i.meth(2)
(1, 3)
```

- The calss produced is exactly like that you'd get from running a **class** statement:

```python
>>> x.__bases__
(<class 'object'>,)
>>> [(a, v) for (a, v) in x.__dict__.items() if not a.startswith('__')]
[('data', 1), ('meth', <function <lambda> at 0x0297A158>)]
```

- Because this **type** call is made automatically at the end of the **class** statement, though, it's an ideal hook for augmenting or otherwise processing a class.
- This trick lies in replacing the default **type** with a custom subclass that will intercept this call. 

[<<< Metaclasses: subclasses of type](102-metaclasses-subclasses-of-type.md) ... [Declaring Metaclassses >>>](104-declaring-metaclass.md)