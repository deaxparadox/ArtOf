# How Properties and Descriptos Relate

Properties and descriptors are strongly related--the property built-in is just a convenient wayt to create a descriptor

to stimulate the **property** built-in with a descriptor class like the following:

```py
class Property:
    def __init__(self, fget=None, fset=None, fdel=None, doc=None):
        self.fget = fget 
        self.fset = fset 
        self.fdel = fdel                    # Save unbound methods
        self.__doc__ = doc                  # or other calls

    def __get__(self, instance, instancetype=None):
        if instance is None:
            return self 
        if self.fget is None:
            raise AttributeError("can't get attribute")
        return self.fget(instance)                          # Pass instance to self
                                                            # in property accessors
    
    def __set__(self, instance, value):
        if self.fset is None:
            raise AttributeError("can't set attribute")
        self.fset(instance, value)

    def __delete__(self, instance):
        if self.fdel is None:
            raise AttributeError("can't delete attribute")
        self.fdel(instance)

class Person:
    def getName(self): print("getName...")
    def setName(self, value): print("setName...")
    name = Property(getName, setName)                       # Use like property

x = Person()
x.name 
x.name = "Bob"

del x.name
```

- This **Property** class catches attribute accesses with the descirptor protocol and routes requests to functions or methods passed in and saved in descriptor state when the class is created

- Attribute fetches, for example, are routed from the **Person** class, to the **Property** class's **__get__** method, and back to the **Person** class's **getName**. With descriptors, this "just works";

```bash
getName...
setName...
Traceback (most recent call last):
  File "/home/creator/Documents/Paradox/Github/ArtOf/ArtOfPython/Examples/managed-attributes/example_14.py", line 35, in <module>
    del x.name
  File "/home/creator/Documents/Paradox/Github/ArtOf/ArtOfPython/Examples/managed-attributes/example_14.py", line 23, in __delete__
    raise AttributeError("can't delete attribute")
AttributeError: can't delete attribute
```



## Descriptors and slots and more

Instance attribute dictionaries are avoided by creating class-level descriptors that intercept slog name access, and map those names to sequential storage space in the instance.