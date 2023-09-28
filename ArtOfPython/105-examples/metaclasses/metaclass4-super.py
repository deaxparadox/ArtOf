# Instances inherit from classes and their supers normally

class SuperMetaObj:
    def __call__(self, classname, supers, classdict):
        print("In SuperMetaObj.call: ", classname, supers, classdict, sep="\n", end="\n\n")
        Class = self.__New__(classname, supers, classdict)
        self.__Init__(Class, classname, supers, classdict)
        return Class

class SubMetaObj(SuperMetaObj):        
    def __New__(self, classname, supers, classdict):
        print("In SubMetaObj.new: ", classname, supers, classdict, sep="\n", end="\n\n")
        return type(classname, supers, classdict)
    
    def __Init__(self, Class, classname, supers, classdict):
        print("In SubMetaObj.init: ", classname, supers, classdict, sep='\n', end="\n\n")
        print("...init class object: ", list(Class.__dict__.keys()))



class Eggs:
    pass 

print("Making class")
class Spam(Eggs, metaclass=SubMetaObj()):          # Invoke Sub instance via Super.__call__
    data = 1                                
    def meta(self, arg):
        return self.data + arg
    
print("Making instance")
X = Spam()
print("data:", X.data, X.meta(2))