<!-- defining class -->
<!-- constructor -->
<!-- instance  -->
<!-- instance variable -->
<!-- class varaible -->


#  Classes

`Classes` are used to create  user-defined data structures. Classes define functions called **methods**, which identity the behaviors and action that an object created from the class can perform with its data.

### How to Define a class

Classes are define using `class` keyword, followed by atributes and methods of class.

We are going to define a empty class **Home**.

```py
class Home:
    pass
```

### Instances

Instances are like a person whose enter the **Home** and access home attributes (here attribute is used to both variables and functions).

Creating instances of **Home**:

```py
h = Home()
```

So, we have create instance `h` of `Home`.


### Define atttribute and function in class

We are going to define class **Home** with attribute name, age, hall(), kitchen(). 


```py
class Home:
    def __init__(self, name, age) -> None:
        self.name = name 
        self.age = age 
    
    def hall(self):
        return "This is hall."
    
    def kitchen(self):
        return "This is kitchen."
```

Creating instance and accessing attribute:

```py
h = Home("Person1", 20)


print(h.name)
print(h.age)
print(h.hall())
print(h.kitchen())
```

```output
Person1
20
This is hall.
This is kitchen.
```

The first method `__init__()` is called constructor function, it is the first object executed by class **Home** when we created an instance. So each instance is like a person with different name and age, and then access house attribute *hall()* and *kitchen()*. Constructor function takes argument *name*, and *age*, which are required for entering, then argument are pass as variable to constructor, which can only be used after creating an instance, and they are called *instance variables*. You can also created contructor function with out any arguments.

In this is we are creating again creating class **Home** with out instance argument.

```py
class Home:

    # constructor function with no argument, 
    # no instance variable
    def __init__(self) -> None:
        pass 

    def hall(self):
        return "This is hall."
    
    def kitchen(self):
        return "This is kitchen."
```

Again we are creating class **Home**, without instance argument, but with *instance varaible*, which we can access after creating instance of class

```py
class Home:
    def __init__(self) -> None:

        # instance variable
        self.house_name = "Super house 1" 

    def hall(self):
        return "This is hall."
    
    def kitchen(self):
        return "This is kitchen."
```

Similar to *instance varible* we can also create *class variable* which are define on class level:

```py
class Home:
    # class variable 
    house_name = "Super house 1" 
    
    def __init__(self) -> None:
        # instance variable
        self.name = "Person1"

    def hall(self):
        return "This is hall."
    
    def kitchen(self):
        return "This is kitchen."
```

We can access *class variable* like this:

```py
print(Home.house_name)
```

```output
Super house 1
```

But for accessing instance variable *self.name* or methods we have to create instance.

```py
h = Home()
print(h.name)
```

```output
Person1
```

We access class methods (function in class are known as methods) in the same way as instance varaible

```py
print(h.hall())
print(h.kitchen())
```

```output
This is hall.
This is kitchen.
```

Methods the same as regular functions, they are argument return object. Now go on testing with them.

[Inheritence >>>](102-Inheritence.md)