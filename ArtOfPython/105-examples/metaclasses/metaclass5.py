# Classes can catch calls to (but built-ins look in metas, not super!)

class SuperMeta(type):
    def __call__(meta, classname, supers, classdict):
        print("In SuperMeta.call: ", classname, supers, classdict, sep='\n', end="\n\n")
        return type.__call__(meta, classname, supers, classdict)
    
    def __init__(Class, classname, supers, classdict):
        print("In SuperMeta init: ", classname, supers, classdict, sep="\n...", end="\n\n")
        print("...init class object: ", list(Class.__dict__.keys()))

print("Making metaclass")
class SubMeta(type, metaclass=SuperMeta):
    def __new__(meta, classname, supers, classdict):
        print("In SubMeta.new: ", classname, supers, classdict, sep="\n...", end="\n\n")
        return type.__new__(meta, classname, supers, classdict)
    
    def __init__(Class, classname, supers, classdict):
        print("In Submeta init:", classname, supers, classdict, sep="\n...", end="\n\n")
        print("...init class object:", list(Class.__dict__.keys()))


class Eggs:
    pass 

print("Making class")
class Spam(Eggs, metaclass=SubMeta):          # Invoke SubMeta, instance via SuperMeta.__call__
    data = 1                                
    def meta(self, arg):
        return self.data + arg
    
print("Making instance")
X = Spam()
print("data:", X.data, X.meta(2))