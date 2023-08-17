# The nonlocal Statement

The **nonlocal** statement is similar in both form and role to **global**. Like **global**, **nonlocal** declares that a name will be changed in an enclosing scope. Unlike **global**, though, **nonlocal** applies to a name in an enclosing function's scope, not the **global** module scope outside all **defs**. Also unlike **global**, **nonlocal** names must already exist in the enclosing function's scope when declared--then can exist only in enclosing functions and cannot be created by a first assignment in a nestef **def**.

**nonlocal** both allows assignment to names in enclosing function scopes and limits scope lookups for such names to enclosing **defs**.

### nonlocal Basics

```py
def func():
    nonlocal name1, name2, ...              # Ok here

>>> nonlocal X
SyntaxError: nonlocal declaration not allowed at module level
```

This statement allows a nest function to change one or more names defined in a syntactically enclosing function's scope.

### nonlocal in Action

```py
>>> 
>>> def tester(start):
...     state = start                       # Referencing nonlocals works normally
...     def nested(label):
...             print(label, state)         # Remembers state in enclosing scope
...     return nested
... 
>>> F = tester(0)
>>> F("spam")
spam 0
>>> F("ham")
ham 0
>>> 
```

### Using nonlocal for changes

If we declare **state** in the **tester** scope as **nonlocal** within **nested**, we got get change it inside the nested function, too. This words even though **tester** has returned and exited by the time we call the returned **nested** function throught the name F:

```py
>>> 
>>> def tester(start):
...     state = start                       # Each call gets it own state
...     def nested(label):
...             nonlocal state              # Remembers state in enclosing scope
...             print(label, state)
...             state += 1                  # Allowed to change it if nonlocal
...     return nested
... 
>>> F = tester(0)
>>> 
>>> F("spam")                               # Increments state on each call
spam 0
>>> F("ham")
ham 1
>>> F("eggs")
```


- We can call the **tester** factory (closure) function multiple times to get multiple times to get mulitple copies of its state in memory. The **state** object in the enclosing scope is essentially attached to the **nested** function object returned; each call makes a new, distinct **state** object, such that updating one function's state won't impact the other.

```py
>>> G = tester(42)          # Make a new tester that starts at 42
>>> G("spam")
spam 42
>>> G('eggs')               # My state information updated to 43
eggs 43
>>> F('bacon')              # But f's is where it left off: at 3
bacon 3                     # Each call has different state information
>>> 
```