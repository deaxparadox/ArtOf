# The Basics

```py
def __getattr__(self, name): ...            # On undefined attribute fetch [object.name]
def __getattribute__(self, name): ...            # On all attribute fetch [object.name]
def __setattr__(self, name, value): ...            # On all attribute fetch [object.name=value]
def __delattr__(self, name): ...            # On undefined attribute fetch [del object.name]
11
```

- `self` is the subject instance object as usual,
- `name` is the string name of the attribute being accessed
- `value` is the object being assigned to the attribute.

For example, to catch every attribute fetch, we can use either of the firs two methods:

```py
class Catcher:
    def __getattr__(self, name):
        print("Get: %s" % name)
    def __setattr__(self, name, value):
        print("Set: %s %s" % (name, value))

X = Catcher()
X.job
X.pay
X.pay = 99

# output 
Get: job
Get: pay
Set: pay 99
```

- Using `__getattribute__` works exactly the same in the specific case:

```py

class Catcher:
    def __getattribute__(self, name: str):
        print("Get: %s" % name)
    def __setattr__(self, name, value):
        print("Set: %s %s" % (name, value))

X = Catcher()
X.job
X.pay
X.pay = 99
```

For example, traces *every* attribute fetch made to another object passed to the wrapper (proxy) class:

```py
class Wrapper:
    def __init__(self, object):
        self.wrapped = object 
    def __getattr__(self, attrname):
        print("Trace: " + attrname)
        return getattr(self.wrapped, attrname)
    
X = Wrapper([1,2,3])
X.append(4)
print(X.wrapped)
```