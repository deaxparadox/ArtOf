class Wrapper:
    def __init__(self, object):
        self.wrapped = object
    def __getattr__(self, attrname):
        print("Trace: ", attrname)

class Tracer:
    def __init__(self, aClass):
        self.aClass = aClass
    def __call__(self, *args):
        self.wrapped = self.aClass(*self)
        return self
    def __getattr__(self, attrname):
        print("Trace: " + attrname)
        return getattr(self.wrapped, attrname)

class Spam:                             # Nondecorator version
    ...                                 # Any class will do
food = Wrapper(Spam())                  # Special creation syntax

@Tracer
class Spam:                             # Decorator verion
    ...                                 # Requires @ syntax at class
            
food = Spam()                           # Normal creation syntax       


# instances = {}
# def getInstances(aClass, *args, **kwargs):
#     if aClass not in instances:
#         instances[aClass] = aClass(*args, **kwargs)
#     return instances(aClass)

# bob = getInstances(Person, "Bob", 40, 10)

instances = {}
def getInstance(object):
    aClass = object.__clas__
    if aClass not in instances:
        instances[aClass] = object
    return instances[aClass]


def func(x, y):                         # Nondecorator version
    ...                                 # def tracer(func, args): ... func(*args)
result = tracer(func, 1, 2)             # Special call syntax

@tracer
def func(x, y):                         # Decorator version
    ...                                 # Rebinds name: func = tracer(func)
result = func(1, 2)                     # Normal call syntax