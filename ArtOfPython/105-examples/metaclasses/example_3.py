from descriptors import Descriptor
from decorators import Decorator

class Base(type):
    def __new__(meta, classname, supers, classdict):
        print("Base.init: ", meta, classname, supers, classdict)
        classdict.update({"runs": 3})
        return type.__new__(meta, classname, supers, classdict)
    

class Add(metaclass=Base):
    value = Descriptor()

    def __init__(self, value: int | None): 
        self.value = value
    
    @Decorator
    def add_one(self):
        self.value += 1
        return self.value

if __name__ == "__main__":
    a = Add(100)
    print(a.value)
    print(a.add_one())
    print(a.runs)