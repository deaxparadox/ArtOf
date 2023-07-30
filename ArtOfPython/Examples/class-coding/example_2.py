class FirstClass:
    def setData(self, value):
        self.data = value
    def display(self):
        print(self.data) 

class SecondClass(FirstClass):          # Inherit setData
    def display(self):                  # Changes display
        print("Current value = '%s'" % self.data)

z = SecondClass()
z.setData(42)               # Finds setdata in FirstClass
z.display()                 # Finds overridden method in SecondClass
