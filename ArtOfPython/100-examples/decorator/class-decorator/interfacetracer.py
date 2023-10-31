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