# Using `__getattribute__`

It catches *all* attributes fetches, this version must be carefull to avoid looping by pasing new fetches to a superclass, and it can't generally assume unknown names are erros:

```py
class Person:
    def __init__(self, name):
        self._name = name                   # Triggers __setattrs__!

    def __getattribute__(self, attr):            # On [obj.undefined]
        print('get: ' + attr)
        if attr == 'name':                                  # Intercept all names
            attr = '_name'                                  # Map to internal name
        return object.__getattribute__(self, attr)          # Avoid looping here
        
    def __setattr__(self, attr, value):         # On [obj.any = value]
        print('set: ' + attr)
        if attr == 'name':
            attr = '_name'                      # Set internal name
        self.__dict__[attr] = value             # Avoid looping here

    def __delattr__(self, attr):                # On [del obj.any]
        print('del: ' + attr)
        if attr == 'name':
            attr = "_name"                      # Avoid looping here too
        del self.__dict__[attr]                 # but much less common

bob = Person("Bob Smith")           # bob has a managed attribute
print(bob.name)                     # Runs __getattr__
bob.name = "Robert Smith"           # Runs __setattr__
print(bob.name)
del bob.name                        # Runs __delattr__

print("-"*20)
sue = Person("Sue Jones")           # sue inherits property too
print(sue.name)

```

- When run with this, the output is similar, but we get an extra `__getattribute__` call for the fetch in `__setattr__` (the first time originating in `__init__`)

```bash
set: _name
get: __dict__
get: name
Bob Smith
set: name
get: __dict__
get: name
Robert Smith
del: name
get: __dict__
--------------------
set: _name
get: __dict__
get: name
Sue Jones
```