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

## Instances

Instances are like a person whose enter the **Home** and access home attributes (here attribute is used to both variables and functions).

Creating instances of **Home**:

```py
h = Home()
```

So, we have create instance `h` of `Home`.


## Define atttribute and function in class

We are going to define class **Home** with attribute name, age, hall(), kitchen(). 


```py
class Home:
    def __init__(self, name):
        self.name = name 
    def welcome_message(self):
        return "Welcome to {}".format(self.name)
    
```

## Creating instance and accessing attribute:

```py
class Home:
    def __init__(self, name):
        self.name = name 
    def welcome_message(self):
        return "Welcome to {}".format(self.name)
    
h = Home("Linux")
print(h.name)
print(h.welcome_message())
```

```bash
Linux
Welcome to Linux
```

- The first method `__init__()` is called `constructor function`, it is the first object executed by class **Home** when we created an instance. Constructor function takes argument *name*, which are required for entering, then argument are pass as variable to constructor, which can only be used after creating an instance, and they are called *instance variables*. You can also created contructor function with out any arguments.

- In this is we are creating again creating class **Home** with out instance argument.


```py
class Home:
    def __init__(self):
        pass
    def welcome_message(self):
        return "Welcome to {}".format(self.name)
    
h = Home()
print(h.welcome_message())
```

```bash
Welcome to Linux
```

## *instance variable*

- Again we are creating class **Home**, without instance argument, but with *instance varaible*, which we can access after creating instance of class

```py
class Home:
    def __init__(self):
        self.name = "Super house 1"
    def welcome_message(self):
        return "Welcome to {}".format(self.name)
    
h = Home()
print(h.name)
print(h.welcome_message())
```

```bash
Super house 1
Welcome to Super house 1
```

## *class varaible*
- Similar to *instance varible* we can also create *class variable* which are define on class level:

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

- We can access *class variable* like this:

```py
print(Home.house_name)
```

```bash
//output 

Super house 1
```

But for accessing instance variable *self.name* or methods we have to create instance.

```py
h = Home()
print(h.name)
```

```output
// output 

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

Methods the same as regular functions, they accept argument and return object. Now go on testing with them.

[Inheritence >>>](102-Inheritence.md)