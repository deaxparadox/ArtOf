# Coding class Basics

## Classes Generate Multiple Instances Objects

There are two kinds of objects in Python's OOP model: *class* objects and *instance* objects

- Class objects provide defualt behavior and serve as factories for instance objects.

- Instances objects are the real objects your programs process--each is a namespace in its own right, but inherites (i.e., has automatic access to) names in the class from which it was created. 

- Classes objects come from statements, and instances comes from calls; each time you call a class, you get a new instance of that class.

- With classes, each instance can have its own independent data, supporting multiple versions of the object that that class models.


## Class Objects Provdie Default Behavior

When we run a `class` statement, we get a class object. Here's a rundown of the main properties of Python classes:

- The class statement creates a class object and assigns it a name.
- Assignments inside class statements make class attributes.
- Class attributes provide object state and behavior.

## Instance Objects are Concrete Items

When we call a class object, we get an instace object. Here's an overview of the key points behind class instances:

- Calling a class object like a function makes a new instance object.
- Each instance object inherits class attributes and gets its own namespace.
- Assignments to attributes of self in methods make per-instance.


## First Example

Let's define a class named **FirstClass** by running a Python class statement:

```py
class FirstClass:                   # Define a class object
    def setData(self, value):       # Define a class's method
        self.data = value           # self is the instance
    def display(self):
        print(self.data)            # self.data: per instance
```

- Functions inside a class are usually called *methods*.
- In a method function the first argument automatically receives an implied instance object when called--the subject of the call.
- let's create few instances:

```py
x = FirstClass()        # Make two instances
y = FirstClass()        # Each is a new namespace
```

- settings data:

```py
x.setData("King Arther")
y.setData(3.14565)
```

- accessing data:

```py
x.display()
y.display()
```

```bash
King Arther
3.14565
```

- If we were to call `display` on one of our instance *before* calling `setData`, we would trigger an undefined name error--the attribute named *data* doesn't even exist in memory until it is assigned within the *setData* method.

Consider the we can change instance attribute in the class ifself, by assigning to self in methods, or *outside* the class, by assigning to an explicit instance object:

```py
>>> x.data = "New Value"
>>> x.display()
New Value
```

We could even generate an entirely *new* attribute in the instance's namespace by assigning to its name outside the class's method functions:

```py
>>> x.anothername = "spam"      # Can set new attributes here too!
```

- This would attach a new attribute called `anothername`, which may or may not be used by any of the class's methods, to the instance object x.