# The Basics

A property is created by assigning the result of a built-in function to a class attribute

attribute = property(fget, fset, fdel, doc)

When these arguments are used, 

- we pass **fget** a function for intercepting attribute fetches, 
- **fset** a  function for assignments
- and **fdel** a function for attribute deletions

This built-in **property** call returns a property object, which we assign to the name of attribute to be managed in the class scope.


## A first Example

```py
class Person:
    def __init__(self, name):
        self._name = name 

    def getName(self):
        print("fetch...")
        return self._name
    
    def setName(self, value):
        print("change...")
        self._name = value 
    
    def delName(self):
        print('remove...')
        del self._name 

    name = property(getName, setName, delName, "name property docs")

def main():
    bob = Person("Bob Smith")
    print(bob.name)
    bob.name = "Robert Smith"
    print(bob.name)
    del bob.name 

    print("-"*20)
    sue = Person("Sue Jones")
    print(sue.name)
    print(Person.name.__doc__)

if __name__ == "__main__":
    main()
```


- This code simple intercepts and trace an attribute--but its serves to demonstrate the protocol.
- When this code is run, two instance inherit the property, just as they would any other attribute attached to their class.

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


Like all class attrbutes, properties are *inherited* by both instance and lower subclasses:

```py
class Lower(Person):
    pass 

def main():
    bob = Lower("Bob Smith")
    print(bob.name)
    bob.name = "Robert Smith"
    print(bob.name)
    del bob.name 

    print("-"*20)
    sue = Lower("Sue Jones")
    print(sue.name)
    print(Person.name.__doc__)

if __name__ == "__main__":
    main()
```

- the output is the same--the **Person** subclass inherits the **name** property from **Super**, and the **bob** instance gets it from **Person**.

## Computed Attributes

For computing the value of an attribute dynamically when fetched:

```py
class PropSquare:
    def __init__(self, start) -> None:
        self.value = start 
    def getX(self):
        return self.value ** 2
    def setX(self, value):
        self.value = value 
    X = property(getX, setX)

def main():
    P = PropSquare(3)
    Q = PropSquare(32)

    print(P.X)
    P.X = 4
    print(P.X)
    print(Q.X)

if __name__ == "__main__":
    main()
```