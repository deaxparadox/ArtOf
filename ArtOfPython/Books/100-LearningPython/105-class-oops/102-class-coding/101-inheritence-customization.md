# Classes are Customized by Inheritence

Class also allow us to make changes by introducing new components (called *subclasses*), instead of changing existing components in place.

In Python, instances inherit from classes, and classes inherit from subclasses:

- Superclasses are listed in parentheses in a class header
- Classes inherit attributes from their superclasses.
- Instances inherit attributes from all accessible classes.
- Each *object.attribute* reference invokes a new, independent search.
- Logic changes are made by subclassing, not by changing superclasses.

## A Second Example


```py
class FirstClass:
    def setData(self, value):
        self.data = value
    def display(self):
        print(self.data) 

class SecondClass(FirstClass):          # Inherit setData
    def display(self):                  # Changes display
        print("Current value = '%s'" % self.data)
```

- In this case, since  the `display` name in `SecondClass` will be found before the one  in `FirstClass`, we say that `SecondClass` *overrides*  `FirsClass`'s `display`.

```py
>>> z = SecondClass()
>>> z.setData(42)               # Finds setdata in FirstClass
>>> z.display()                 # Finds overridden method in SecondClass
Current value = '42'
```