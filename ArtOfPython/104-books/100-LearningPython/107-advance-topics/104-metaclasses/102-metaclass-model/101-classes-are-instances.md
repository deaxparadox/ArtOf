# Classes are Instaces of type

- In Python 3.X, use defined calss objects are instances of the object named *type*, which is itself a class.
- In Python 2.X, new style classes inherit from *object*, which is a subclass of *type*; classic classes  are instances of *type* and are not created from a class.

The *type* build-in returns the type of any object (which is itself an object) when called with a single argument.

- For built-in types like lists, the type of the instance is the built-in list type, but the type of the list type is the type *type* itself--the *type* object at the top of the hierarchy creates specific types, and specific types creates instances.

```python
In [4]: print(type([]))
<class 'list'>

In [5]: print(type(type))
<class 'type'>

In [6]: print(type(type(list)))
<class 'type'>
```

- the type/instance relationship holds true for user-defined classes as well: instances are created from classes, and classes are created from *type*.

- In Python 3.X, the notion of a "type" is mergest withe notion of a "class".
- The two are essentially synonyms -- *classes are types, and types are classes*:

    - Types are defined by classes that derive from *type*.
    - User-defined classes are instances of tyep classes.
    - User-defined calsses are types that generate instances of their own.

- The type of an instance is the class from which it was generated.

```python
In [9]: class C:
   ...:     pass
   ...: 

In [10]: C.__class__
Out[10]: type

In [11]: x = C()

In [12]: x.__class__
Out[12]: __main__.C

In [13]: type(x)
Out[13]: __main__.C

In [14]: 

In [14]: type(C)
Out[14]: type

In [15]: 
```

- classes are not really a separate concept at all: they are simply user-defined types, aand *type* itself is defined by a class.

[<<< README](README.md) ... [Metaclasses: Subclasses of type >>>](102-metaclasses-subclasses-of-type.md)