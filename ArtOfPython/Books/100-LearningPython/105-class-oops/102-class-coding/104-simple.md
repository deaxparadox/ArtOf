# The World's Simplest Python Class

The following statements makes a class with no attributes attache, an empty namespace object:

```py
>>> class rec: pass             # Empty namespace object
... 
>>> 
```

- we will attach attributes to the class by assigning names to it outside of the original `class` statement:

```py
>>> rec.name = "Bob"
>>> rec.age = 40
>>> 
```

And, after we've created these attrbiutes by assignments, we can fetch them with the usual syntax:

```py
>>> print(rec.name)
Bob
```

<mark>
    Notice that this works even though there are *no instances* of the class yet; classes are objects in their own right, even without instances.
</mark>


In fact, there are just self-contained namespaces; as long as we have a reference to a class, we can set or change its attributes anytime we wish.

- Watch what happens when we do create two instances:

```py
>>> 
>>> x = rec()
>>> y = rec()
>>> 
```

- But they inherit the attributes we attached to the class by inheritence:

```py
>>> x.name, y.name
('Bob', 'Bob')
>>> 
```

- <mark>attribute <i>references</i> kick off inheritence searches, but attribute <i>assignments</i> affect only the object in which the assignments are made.</mark>

- this mean `x` get its own `name`, but `y` still inherits the name attached to the class above it:

```py
>>> x.name = "Sue"                  # But assignment changes x only
>>> rec.name, x.name, y.name
('Bob', 'Sue', 'Bob')
>>> 
```

<mark>The <strong>__dict__</strong> attribute is the namespace dictionary for most class-based objects.</mark>


```py
>>> list(rec.__dict__.keys())
['__module__', '__dict__', '__weakref__', '__doc__', 'name', 'age']
>>> 
```

To facilitate inheritence search on attribute fetches, each instance has a link to its class that Python creates for us--it's called `__class__`:

```py
>>> x.__class__
<class '__main__.rec'>
>>> 
```

Classes also have a `__bases__` attribute, which is a tuple of refrences to their superclasses objects:

```py
>>> rec.__bases__
(<class 'object'>,)
>>> 
```

### Fully implemented example of class

```py
class Person:
    def __init__(self, name, jobs, age=None):
        self.name = name 
        self.jobs = jobs 
        self.age = age 
    def info(self):
        return (self.name, self.jobs)
    
rec1 = Person("Bob", ["div", "mgr"], 40.5)
rec2 = Person("Sue", ["dev", "cto"])

print(rec1.jobs)
print(rec2.info())
```

```bash
['div', 'mgr']
('Sue', ['dev', 'cto'])
```