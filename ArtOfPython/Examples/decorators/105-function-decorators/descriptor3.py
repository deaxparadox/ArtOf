class tracer(object):                           # For methods, but not functions!
    def __init__(self, meth):                   # On @ decorator
        self.calls = 0
        self.meth = meth 
    def __get__(self, instance, owner):         # On method fetch
        def wrapper(*args, **kwargs):           # On method call: proxy with self+inst
            self.calls += 1
            print("call %s to %s" % (self.calls, self.meth.__name__))
            return self.meth(instance, *args, **kwargs)
        return wrapper
    
class Person:                               # Applies to class methods
    @tracer                                 # giveRaise = tracer(giveRaise)
    def giveRaise(self, percent):           # Makes giveRaise a descriptor
        ...

@tracer                         # But fails for simple functions 
def spam(a, b, c):              # spam = tracer(spam)
    ...                         # No attribute fetch occurs here