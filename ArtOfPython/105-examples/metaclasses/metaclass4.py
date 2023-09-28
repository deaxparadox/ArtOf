# A normal class instance can serve as a metaclass to.

class MetaObj:
    def __call__(self, classname, supers, classdict):
        print("In MetaObj.call: ", classname, supers, classdict, sep="\n", end="\n\n")
        Class = self.__New__(classname, supers, classdict)
        self.__Init__(Class, classname, supers, classdict)
        return Class 
    
    def __New__(self, classname, supers, classdict):
        print("In MetaObj.new: ", classname, supers, classdict, sep="\n", end="\n\n")
        return type(classname, supers, classdict)
    
    def __Init__(self, Class, classname, supers, classdict):
        print("In MetaObj.init: ", classname, supers, classdict, sep='\n', end="\n\n")
        print("...init class object: ", list(Class.__dict__.keys()))

class Eggs:
    pass 

print("Making class")
class Spam(Eggs, metaclass=MetaObj()):          # Metaobj is normal class instance
    data = 1                                    # Called at end of statement
    def meta(self, arg):
        return self.data + arg
    
print("Making instance")
X = Spam()
print("data:", X.data, X.meta(2))