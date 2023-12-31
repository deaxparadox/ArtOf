# State Retention Options

- The **nonlocal** statement allows multiple copies of *changeable* state to be retained in memory.

## State with nonlocal: 3.X only

The following code alllows state to be retained and modified in an enclosing scope. Each call to **tester** creates a self-contained *package of changeable informtaion*, whose names do not clash with any other part of the program:

```py
>>> 
>>> def tester(start):
...     state = start                   # Each call gets its own value
...     def nested(label):
...             nonlocal state          # Remembers state in enclosing scope
...             print(label, state)
...             state += 1              # Allowed to chagne it if nonlocal
...     return nested
... 
>>> F = tester(0)
>>> F("spam")                           # State visible within closure only
spam 0
>>> F.state
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: 'function' object has no attribute 'state'
>>> 
```

We need to declare varaibles nonlocal only if they must be changed (other enclosing scope name references are automatically retained as usual), and nonlocal names are still not visible outside the enclosing function.

## State with Globals: A Single Copy Only

```py
>>> 
>>> def tester(start):
...     global state                    # Move it out to the module to change it
...     state = start                   # global allows changes in modules scope
...     def nested(label):
...             global state
...             print(label, state)
...             state += 1
...     return nested
... 
>>> F = tester(0)
>>> 
>>> F("spam")                           # Each call increments shared global state
spam 0
>>> F('eggs')
eggs 1
>>> 
```

This works in this case, but it requires **global** declarations in both functions and is prone to name collisions in the global scope. A worse, and more subtle, problem is that it only allows for a *single shared copy* of the state information in the module scope--if we call **tester** again, we'll wind up resetting the module's **state** variable, such that prior calls will see their state overwritten:

```py
>>> 
>>> G = tester(42)                  # Resets state's single copy in global scope
>>> G("toast")
toast 42
>>> G("bacon")
bacon 43
>>> F("ham")                        # But my counter has been overwritten!
ham 44
>>> 
```

- When you are using **nonlocal** and nested function closures instead of **global**, each call to **tester** remembers its own unique copy of the **state** object.

## State with Classes: Explicit Attributes (Preview)

*classes with atributes* to make state information access more explicit then the implicit magic of scope lookup rules. As an added benefit, each instance of a class gets a fresh copy of state information, as natural by product of Python's object model

The function named `__init__` is run automatically when the class is called:

```py
class tester:
    def __init__(self, start):
        self.start = start 

    def nested(self, label):
        print(label, self.start)
        self.start += 1

F = tester(0)
F.nested("spam")
F.nested("ham")
```

```
spam 0
ham 1
```

In classes, we save *every* attribute explicitly, whether it's changed or just referenced, and they are available outside the class. The class also supports mulitple copies of the retained data.

```py
>>> g = tester(0)
>>> g.nested("ham")
ham 0
>>> g.nested("spam")
spam 1
>>> 
```

----------

We could also make our class objects look like callable functions using using operator overloading. `__call__` intercepts direct calls on an insstnace, so we don't need to call a named method:

```py
    class tester:
        def __init__(self, start) -> None:
            self.start = start
        def __call__(self, label):
            print(label, self.start)
            self.start += 1

    H = tester(99)
    H("juice")                          # Invokes __call__
    H("pancakes")

```

```
juice 99
pancakes 100
```

## State with Function Attributes

When you attach user-defined attributes to nested functions generated by enclosing factory functions, they can also serve as per-call, multiple copy, and writable state, just like nonlocal scope closures and class attributes. Such user-defined attributes names won't clash with names Python creates itself, and as for **nonlocal**, need be used only for state variables that must be *changed*; other scope references are retained and work manually.

Because factory functions make a new functionss on each call anyhow, this does not require extra objects--the new function's attributes become per-call state in much the same way as nonlocals, and are similarly associated with the generated function in memory.

Moreover, function attributes allow state variables to be accessed *outside* the nested function, like class attributes; with **nonlocal**, state variables can be seen directly only within the nested **def**. If you need to access a call counter externally, it's a simple function  attribute fetch in this method.

```py
    def tester(start):
        def nested(label):
            print(label, nested.state)
            nested.state += 1
        nested.state = start 
        return nested

    F = tester(0)
    F("spam")
    F("ham")
    print(F.state)
```


```bash
spam 0
ham 1
2
```

Because each call to the outer function produces a new nested function object, this scheme supports mulitple copy **per-call** changeable data just like nonlocal closures and classes--a usage mode that global variables cannot provide:

```py
>>> G = tester(42)
>>> G("eggs")
eggs 42
>>> F = tester(0)
>>> F("spam")
spam 0
>>> F("ham")
ham 1
>>> F.state
2
>>> F("ham")
ham 2
>>> 
>>> G.state
43
>>> F is G
False
>>> 
```

This code relies on the fact that the function name **nested** in a local variable in the **tester** scope enclosing **nested**; as such it can be referenced freely inside **inside**. . This code also relies on the fact that changing an object in place is not an assignment to a name; when it increments nested.state, it is  changing part of the object nested references, not the name  nested itself. Because we’re not really assigning a name in the enclosing scope, no nonlocal declaration is required.
