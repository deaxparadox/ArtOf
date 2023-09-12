class Decorator:
    def __init__(self, C):                          # On @ decoration
        self.C = C 
    def __call__(self, *args):                      # On instance creation
        self.wrapped = self.C(*args)
        return self 
    def __getattr__(self, attrname):                # On attribute fetch
        return getattr(self.wrapped, attrname)
    
@Decorator
class C: ...                            # C = Decorator(C)

x = C()
y = C()                                 # Overwrites x!