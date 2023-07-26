# Using State Information in Descriptors

First Example: 

- it uses data stored on the client instance

```py
class Name:
    "name descirptor docs"
    def __get__(self, instance, owner):
        print("fetch...")
        return instance._name
    def __set__(self, instance, value):
        print("change...")
        instance._name = value 
    def __delete__(self, instance):
        print("remove...")
        del instance._name

class Person:
    def __init__(self, name):
        self._name = name 

    name = Name()
```


Square example:

- uses data attached to the *descriptor* object itself (a.k.a self)

```py
class DescSquare:
    def __init__(self, start):              # Each descriptor has own state
        self.value = start 
    def __get__(self, instance, owner):     # On attr fetch
        return self.value ** 2 
    def __set__(self, instance, value):     # On attr asign
        self.value = value                  # No delete or docs


class Client1:
    X = DescSquare(3)

```


Descriptors can use both instance state and descriptor state, or any combination thereof:


- *Descriptor state* is used to managed either data internal to the workings of the descriptor, or data that spans all instances. It can vary per instance apperance (often per client class). Therefore descriptor state is **per-descriptor data**
- *Instance state* records information related to and possibly created by the client class. It can vary per client class instance (that is, per application object). Therefore instance state is **per-client-instance data**

## **Note**

- You would not use *descriptor* state to record employee names, since each client instance requires its own value--if stored in the descriptor, each client class instance will effectively stare the same single copy.
- On the other hand, you would not usually use *instance* state to record data pertaining implementation internals--if stored in each instance, there would be multiple varying copies.

The following descriptor attaches information to its own instance, so it doesn't class with that on the client class's instance--but also shares that information between two client instances:

```py
class DescState:
    def __init__(self, value):
        self.value = value 
    def __get__(self, instance, owner):
        print("DescState get")
        return self.value * 10
    def __set__(self, instance, value):
        print("DescState set")
        self.value = value 

class CalcAttrs:
    X = DescState(2)                    # Desciptor class attr
    Y = 3                               # Class attr
    def __init__(self):
        self.Z = 4                      # Instance attr


obj = CalcAttrs()
print(obj.X, obj.Y, obj.Z)              # X is compyuted, others are not
obj.X = 5                               # X assignment is intercepted
CalcAttrs.Y = 6                         # Y reassigned in class
obj.Z = 7                               # Z assigned in instance
print(obj.X, obj.Y, obj.Z)

obj2 = CalcAttrs()
print(obj2.X, obj2.Y, obj2.Z)
```

```bash
DescState get
20 3 4
DescState set
DescState get
50 6 7
DescState get
50 6 4
```

It's also feasible for a descriptor to store or use an attribute attached to the client class's *instance* instead of itself.

- This allows for data that can very per client class instance.
- the descriptor in the following example assumes the instance has an attribute **_X** attached by the client class, and uses it to compute the value of the attribute it represents:

```py
class InstState:
    def __get__(self, instance, owner):
        print("Instance get")
        return instance._X * 10
    def __set__(self, instance, value):
        instance._X = value


class CalcAttrs:
    X = InstState()                    # Desciptor class attr
    Y = 3                               # Class attr
    def __init__(self):
        self._X = 2
        self.Z = 4                      # Instance attr


obj = CalcAttrs()
print(obj.X, obj.Y, obj.Z)              # X is compyuted, others are not
obj.X = 5                               # X assignment is intercepted
CalcAttrs.Y = 6                         # Y reassigned in class
obj.Z = 7                               # Z assigned in instance
print(obj.X, obj.Y, obj.Z)

obj2 = CalcAttrs()
print(obj2.X, obj2.Y, obj2.Z)
```


```bash
Instance get
20 3 4
Instance get
50 6 7
Instance get
20 6 4
```

- **X** is assigned to a descriptor as before that manages accesses.
- The new descriptor here, though, has no information itself, but it uses an attribute assumed to exist in the instance--that attribute is name **_X**

This is the general advantage t hat descriptors have over properties-because they have state of their own, they can easily retain data internally, without adding it to the namespace of the client instance object.


The following uses *both* state sources--its **self.data** retains per-attribute information, while its **instance.data** can vary per client instance:

```py
class DescBoth:
    def __init__(self, data):
        self.data = data 
    def __get__(self, instance, owner):
        return "%s %s" % (self.data, instance.data)
    def __set__(self, instance, value):
        instance.data = value 

class Client:
    def __init__(self, data):
        self.data = data 
    managed = DescBoth("spam")

I = Client("eggs")
print(I.managed)            # Show both data sources
I.managed = "SPAM"
print(I.managed)            # Change instance data
```

```bash
spam eggs
spam SPAM
```