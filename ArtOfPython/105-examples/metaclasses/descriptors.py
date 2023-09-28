
class Descriptor:
    def __set_name__(self, owner, name):
        self.public_name = name 
        self.private_name = "__" + self.public_name
    def __get__(self, obj, objtype=None):
        return getattr(obj, self.private_name)
    
    def __set__(self, obj, value):
        return setattr(obj, self.private_name, value)

class Number:
    a = Descriptor()
    b = Descriptor()
    def __init__(self, a, b) -> None:
        self.a = a
        self.b = a