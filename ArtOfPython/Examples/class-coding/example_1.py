class FirstClass:                   # Define a class object
    def setData(self, value):       # Define a class's method
        self.data = value           # self is the instance
    def display(self):
        print(self.data)            # self.data: per instance

x = FirstClass()        # Make two instances
y = FirstClass()        # Each is a new namespace

x.setData("King Arther")
y.setData(3.14565)


x.display()
y.display()

x.data = "New Value"
x.display()

x.anothername = "spam"