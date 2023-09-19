# Coding Properties with Decorators

The **property** built-in can serve as a decorator, to define a function that will run automatically when an attribute is fetched:

```py
class Person:
    @property
    def name(self): ...     # Rebinds: name = property(name)
```

- When run, the decorated method is automatically passed to the first argument of the **property** built-in. 

- Alternatively

```py
class Person:
    def name(self): ...    
    name = property(name)
```

## Setter and deleter decorators

property objects also have **getter**, **setter**, and **deleter** methods that assign the corresponding property accessor methods and return a copy of the property itself.


```py
class Person:
    def __init__(self, name):
        self._name = name 

    @property
    def name(self):             # name = property(name)
        "name property docs"        
        print("fetch...")
        return self._name 
    
    @name.setter
    def name(self, value):      # name = name.setter(name)
        print("change...")
        self._name = value 
    
    @name.deleter               # name = name.deleter(name)
    def name(self):
        print("remove...")
        del self._name 

def main():
    bob = Person("Bob Smith")   # bob has a managed attribute
    print(bob.name)             # Runs name getter (name 1)
    bob.name = "Robert Smith"   # Runs name setter (name 2)
    print(bob.name)
    del bob.name                # Runs name deleter (name 3)

    print("-"*20)
    sue = Person("Sue Jones")
    print(sue.name)             # sue inherits property too
    print(Person.name.__doc__)  # Or help (Person.name)

if __name__ == "__main__":
    main()
```

```bash
fetch...
Bob Smith
change...
fetch...
Robert Smith
remove...
--------------------
fetch...
Sue Jones
name property docs
```