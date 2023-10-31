# Applying class Decorators to built-in types

Previously used decorator:

```py
def Tracer(aClass):                                 # On @ decorator
    class Wrapper:
        def __init__(self, *args, **kwargs):        # On instance creation
            self.fetches = 0
            self.wrapped = aClass(*args, **kwargs)  # Use enclosing scope name
        def __getattr__(self, attrname):
            print("Trace:", attrname)               # Catches  all but own attrs
            self.fetches += 1
            return getattr(self.wrapped, attrname)  # Delegate to wrapped obj
    return Wrapper
```



We can also use the decorator to wrap up a built-in type such as a list, as long as we either subclass to allow decoration or perform the decoration manually--decorator syntax requires a **class** statement for the @ line. 

In the following, **x** is really a **Wrapper** againg due to the indrection of decoration:

```py
```