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