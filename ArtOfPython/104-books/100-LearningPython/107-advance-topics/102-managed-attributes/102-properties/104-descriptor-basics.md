# Descriptors: The Basics

```py
class Descriptors:
    """docstring goes here"""
    def __get__(self, instance, owner): ...         # Return attr value
    def __set__(self, instance, value): ...         # Return nothing (None)
    def __delete__(self, instance): ...             # Return nothing (None)
```

- Classes with any of these methods are considered descriptors, and their methods are special when one of their intances is assigned to another class's  attribute when the attribute is accessed, they are automatically invoked.

- If any of these method are absent, it generally means thay the corresponding type of access is not supported.

- Unlike properties however, omitting a `__set__` allows the descriptor attribute's named to be assigned and thus redefined in an instance, thereby *hiding* the descriptor--to make in attribute *read-only*, you must define `__set__` to catch assginments and raise an exception.

- a descriptor with a `__set__` is konwn formatlly as *data descriptor*, and is given precedence over other names located by normal inheritence rules.

## Descriptor method arguments

- All three descriptors method soutlined in the prior section are passed both the descriptor class instance (**self**), and the instance of the client class to which the descriptor instance is attached (**instance**).

- The `__get__` access method additionally recevies an **owner** argument, specifing the class to which descriptor instance is attached.

    - Its instance argument is either the instance throught which the attribute was accessed (for ***isntance*.attr**), or *None* when the attirbute is accessed through the onwner class directly (for ***class*.attr**).


```py
class Descriptor:
    def __get__(self, instance, owner):
        print(self, instance, owner, sep="\n")

class Subject:
    attr = Descriptor()
    

def main():
    X = Subject()
    X.attr

    print("-"*20)

    Subject.attr

if __name__ == "__main__":
    main()
```

## Read-only descriptors

Simply omitting the **__set__** method in a descriptor isn't enough to make an attribute read-only, because the descriptor name can be assigned to an instance.

In the following, the attribute assignment to **X.a** stores **a** in the instance object **X**, thereby hiding the descriptor stored in class **C**.

```py
>>> class D:
...     def __get__(*args): print("get")
... 
>>> class C:
...     a = D()             # Attribute a is descirptor instance
... 
>>> 
>>> X = C()
>>> X.a
get
>>> C.a
get
>>> X.a = 00
>>> X.a
0
>>> X.a = 99
>>> X.a
99
>>> list(X.__dict__.keys())
['a']
>>> Y = C()                 # Y still inherits descriptor
>>> Y.a
get
>>> C.a
get
>>> 
```

To make a descriptor-based attribute read-only, catch the assignement in the descriptor class and raise an exception to prevent attribute assignment--when assigning an attribute that is a descriptor, Python effectively bypasses the normal instance-level assignment behaviour and routes the operation to the descriptor object:

```python
>>> 
>>> class D:
...     def __get__(*args): print("get")
...     def __set__(*args): raise AttributeError("cannot set")
... 
>>> class C:
...     a = D()
... 
>>> X = C()                             # Routed to C.a.__get__
>>> X.a
get
>>> X.a = 99                            # Routed to C.a.__set__
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "<stdin>", line 3, in __set__
AttributeError: cannot set
>>> 
```
