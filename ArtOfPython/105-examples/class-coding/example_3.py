class FirstClass:
    def setData(self, value):
        self.data = value
    def display(self):
        print(self.data) 

class SecondClass(FirstClass):
    def display(self):
        print("Current value = '%s'" % self.data)

class ThirdClass(SecondClass):                      # Inherited from SeconClass
    def __init__(self, value):                      # On "ThirdClass(value)"
        self.data = value 
    def __add__(self, other):                       # On "self + other"
        return ThirdClass(self.data + other)
    def __str__(self):                              # On "print(self)", "str()"
        return '[ThirdClass: %s]' % self.data
    def mul(self, other):                           # In-place change: named
        self.data *= other

