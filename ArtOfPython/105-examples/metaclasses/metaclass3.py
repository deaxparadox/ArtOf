# A Simple function can serve as a metaclass too

def MetaFunc(classname, supers, classdict):
    print("In MetaFunc: ", classname, supers, classdict, sep="\n...")
    return type(classname, supers, classdict)

class Eggs:
    pass 

print("making class")
class Spam(Eggs, metaclass=MetaFunc):       # Run simple function at end
    data = 1                                # Function returns class
    def meta(self, arg):
        return self.data + arg
    
print("Making instance")
X = Spam()
print("Data: ", X.data, X.meta(2))